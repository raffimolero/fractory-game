mod collision;
#[cfg(test)]
mod tests;

use super::{
    fractal::Fractal,
    orientation::Transform,
    path::TilePos,
    tile::{Quad, Tile},
};
use std::{
    collections::{BTreeSet, HashSet},
    fmt::{Debug, Display},
};

/// a complete action that can be done to the tree,
/// where T is a position that is either relative (TileOffset)
/// or absolute (TilePos)
#[derive(Debug, Clone, Copy)]
pub struct TargetedAction<T> {
    pub target: T,
    pub act: TileAction<T>,
}

/// action to do at an exact node
#[derive(Debug, Clone, Copy)]
pub enum TileAction<T> {
    /// moves this fragment to another tile
    Move(T, Transform),

    /// stores this fragment in the player's inventory
    Store,

    // /// places a fragment onto the fractal from the player's inventory
    // Place(FragmentId),
    /// activates this tile in the next tick
    Activate,
}

type Index = usize;

// TODO: figure out if you can merge Fractal with CollisionCleaner (Node) and DeadEndCleaner (hidden in collision::clean_dead_ends)
// TODO: move self::{self, collision} to Actions?

#[derive(Clone, PartialEq, Eq, Default)]
pub enum Node {
    #[default]
    Free,
    Bad,
    Leaf(Index),
    Branch(Box<Quad<Self>>),
}

impl Node {
    pub fn create_at(mut path: TilePos, value: Index) -> Self {
        match path.pop_outer() {
            Some(subtile) => {
                // Node does not implement Copy, hardcoding 4 frees is easier.
                let mut children = Quad([Node::Free, Node::Free, Node::Free, Node::Free]);
                children[subtile] = Self::create_at(path, value);
                Self::Branch(Box::new(children))
            }
            None => Self::Leaf(value),
        }
    }

    /// a workaround for Drop which allows mutating a shared data structure
    fn drop_with(&mut self, drop_item: &mut impl FnMut(Index)) {
        match std::mem::replace(self, Node::Bad) {
            Node::Free => {}
            Node::Bad => {}
            Node::Leaf(item) => drop_item(item),
            Node::Branch(children) => {
                for mut node in children.0 {
                    node.drop_with(drop_item);
                }
            }
        }
    }

    /// sets a specified value at a specified path.
    /// calls drop_item if a collision happens.
    pub fn set(&mut self, mut path: TilePos, value: Index, drop_item: &mut impl FnMut(Index)) {
        let mut reject = |this: &mut Self| {
            drop_item(value);
            this.drop_with(drop_item);
        };

        match self {
            Node::Free => *self = Self::create_at(path, value),
            Node::Bad | Node::Leaf(_) => reject(self),
            Node::Branch(children) => match path.pop_outer() {
                Some(subtile) => children[subtile].set(path, value, drop_item),
                None => reject(self),
            },
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Free => write!(f, "."),
            Node::Bad => write!(f, "X"),
            Node::Leaf(val) => write!(f, "{val}"),
            Node::Branch(children) => {
                write!(f, "{{ ")?;
                for child in &children.0 {
                    write!(f, "{child}")?;
                    write!(f, " ")?;
                }
                write!(f, "}}")
            }
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Free => write!(f, "."),
            Node::Bad => write!(f, "X"),
            Node::Leaf(val) => write!(f, "{val:?}"),
            Node::Branch(children) => {
                write!(f, "{{ ")?;
                for child in &children.0 {
                    write!(f, "{child:?}")?;
                    write!(f, " ")?;
                }
                write!(f, "}}")
            }
        }
    }
}

/// builds a quadtree from braces, values, and dots
/// ```ignore
/// let tree = tree! ({
///     { .  () () .  }
///     { () () .  () }
///     { }
///     .
/// });
/// println!("{tree:?}");
/// ```
macro_rules! tree {
    (.) => {
        Node::Free
    };
    (X) => {
        Node::Bad
    };
    ({ $a:tt $b:tt $c:tt $d:tt }) => {
        Node::Branch(Box::new(Quad([tree!($a), tree!($b), tree!($c), tree!($d)])))
    };
    ($t:expr) => {
        Node::Leaf($t)
    };
}
pub(crate) use tree;

/// temporary struct to represent a bunch of moves
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RawMoveList {
    pub moves: Vec<(TilePos, (TilePos, Transform))>,
}

// TODO: figure out how to make the coupling with the fractal quadtree clearer,
// because Fractal <- RawMoveList <- Node<LeafItem> and the dependence is clear

impl RawMoveList {
    pub fn add(&mut self, from: TilePos, to: TilePos, transform: Transform) {
        self.moves.push((from, (to, transform)));
    }

    /// applies all the moves, resolving conflicts on the way,
    /// and returning only the moves that were executed.
    pub fn apply(mut self, tree: &mut Fractal) -> CleanMoveList {
        self.clean_sources(tree);
        self.clean_forks();
        self.clean_merges();
        self.clean_dead_ends(tree);
        CleanMoveList { inner: self }
    }

    fn clean_sources(&mut self, tree: &Fractal) {
        let mut set = HashSet::new();
        let mut i = 0;
        while let Some(item @ (src, _dst)) = self.moves.get(i).copied() {
            if set.insert(item) && tree.get_info(tree.get(src).id).fill.is_full() {
                i += 1;
            } else {
                self.moves.swap_remove(i);
            }
        }
    }

    fn clean_forks(&mut self) {
        let mut tree = Node::default();
        let mut holes = BTreeSet::new();
        for (i, (src, _dst)) in self.moves.iter().copied().enumerate() {
            tree.set(src, i, &mut |idx| {
                holes.insert(idx);
            });
        }
        for idx in holes.into_iter().rev() {
            self.moves.swap_remove(idx);
        }
    }

    fn clean_merges(&mut self) {
        let mut tree = Node::default();
        let mut holes = BTreeSet::new();
        for (i, (_src, (dst, _tf))) in self.moves.iter().copied().enumerate() {
            tree.set(dst, i, &mut |idx| {
                holes.insert(idx);
            });
        }
        for idx in holes.into_iter().rev() {
            self.moves.swap_remove(idx);
        }
    }

    fn clean_dead_ends(&mut self, main_fractal: &mut Fractal) {
        /*
        "Valid until proven otherwise."

        move is invalid if:
        - destination contains a single invalid tile

        tile is invalid if:
        - it is non empty
        - it is not moved out

        goal: build a chain where each dependency knows how to invalidate its dependents
        think of moves not as "source->destination", but "dependent<-dependency"
        dependencies will store which dependents they will invalidate.

        think of the simple case where there are only 2 moves. one after the other.
        no 2 sources nor 2 destinations will overlap.

        if a destination is under a source, it is free (until the source is marked bad)
        if a destination is at a source, it is free (until the source is marked bad)
        if a destination lands on a parent of a source, it is free (until one of the sources is marked bad)
        if a source is under a destination, it is uhh idk

        dst:
        - valid if empty
        - invalid if blocked
        - valid if blocker is src
        - invalid if blocker src is invalidated

        "in-place" method
        - create a src tree
            - mark all src in a separate tree with their current values
            - set all src in main fractal to empty (important; auto-merges zero'd tiles)
        - invalidate all dst that don't lead to empty
            - find the src
            - reset the tile at src to previous value
            - invalidate anything above or under the src
                - iterate down the path of src
                - at every step, find empty slots with a dst tag(?)
                - invalidate that dst (depth or breadth?)

                    * questionable: depth first. otherwise we will lose aux data related to backtracking.
                      especially important when a sparse tringle becomes empty after a few moves,
                      but one of the moves is invalidated.
                    * ok but what about a fractal being shrunk? will it invalidate its own dst?

        required slot aux data:
            - if dst:
                - src
                - prev value (keep current value, just mark src?)

        NOTE: must still produce a move list to store the animations which will be used for the UI
        */

        // this is an in-place algo btw

        #[derive(Debug, Default)]
        enum Tree<T> {
            #[default]
            Free,
            Leaf(T),
            Branch(Box<Quad<Self>>),
        }

        // let mut srcs = Tree::Free;
        let mut dsts = Tree::Free;

        // take out all the source tiles
        let mut old_tiles = vec![];
        for (i, (src, (dst, _tf))) in self.moves.iter().copied().enumerate() {
            let old_tile = main_fractal.set(src, Tile::SPACE);
            debug_assert_ne!(old_tile, Tile::SPACE);
            debug_assert!(main_fractal.get_info(old_tile.id).fill.is_full());
            old_tiles.push(old_tile);
            dsts.set(dst, i);
        }

        // mark dead ends as dead
        let mut dead = vec![];
        for (i, (_src, (dst, _tf))) in self.moves.iter().copied().enumerate() {
            if main_fractal.get(dst) != Tile::SPACE {
                dead.push(i);
            }
        }

        // invalidate dead ends and mark their dependents
        // preserve ordering
        while let Some(i) = dead.pop() {
            if old_tiles[i] == Tile::SPACE {
                continue;
            }
            let (src, _dst_tf) = self.moves[i];
            main_fractal.set(src, old_tiles[i]);
            old_tiles[i] = Tile::SPACE;
            dsts.invalidate(src, &mut |i| dead.push(i));
        }

        // execute all the moves, take out failed ones, retain working ones
        let mut out = vec![];
        for (mv @ (_src, (dst, tf)), tile) in self.moves.drain(..).zip(old_tiles) {
            if tile == Tile::SPACE {
                continue;
            }
            main_fractal.set(dst, tile + tf);
            out.push(mv);
        }
        self.moves = out;

        impl<T> Tree<T> {
            fn set(&mut self, mut pos: TilePos, val: T) {
                assert!(
                    !matches!(self, Tree::Leaf(_)),
                    "should be no collisions at this point"
                );

                let Some(subtile) = pos.pop_outer() else {
                    *self = Tree::Leaf(val);
                    return;
                };

                if let Tree::Free = self {
                    *self = Tree::Branch(Box::new(Quad([
                        Tree::Free,
                        Tree::Free,
                        Tree::Free,
                        Tree::Free,
                    ])));
                };

                let Tree::Branch(children) = self else {
                    unreachable!();
                };
                children[subtile].set(pos, val);
            }
        }

        // DstTree
        // dsttree will be indexed by src
        impl Tree<Index> {
            fn invalidate(&mut self, mut pos: TilePos, drop_item: &mut impl FnMut(Index)) {
                match self {
                    Tree::Free => {}
                    Tree::Leaf(_) => self.drop_with(drop_item),
                    Tree::Branch(children) => match pos.pop_outer() {
                        Some(subtile) => children[subtile].invalidate(pos, drop_item),
                        None => self.drop_with(drop_item),
                    },
                }
            }

            fn drop_with(&mut self, drop_item: &mut impl FnMut(Index)) {
                match std::mem::replace(self, Tree::Free) {
                    Tree::Free => {}
                    Tree::Leaf(val) => drop_item(val),
                    Tree::Branch(children) => {
                        for mut child in children.0 {
                            child.drop_with(drop_item);
                        }
                    }
                }
            }
        }

        // SrcTree
        impl Tree<TilePos> {}

        // let mut dsts = DeadEndTree::default();
        // let mut srcs = DeadEndTree::default();
        // let mut ends = HashSet::new();

        // for (src, dst) in self.moves.iter().copied() {
        //     ends.insert(dst);
        //     dsts.set(dst, src, &mut |_| unreachable!("destination overlaps"));
        //     srcs.set(src, dst, &mut |_| unreachable!("source overlaps"));
        // }

        // for (src, dst) in self.moves.iter().copied() {
        //     // dst
        // }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CleanMoveList {
    inner: RawMoveList,
}

impl CleanMoveList {
    pub fn moves(&self) -> &[(TilePos, (TilePos, Transform))] {
        &self.inner.moves
    }
}

// TODO: double check all pub visibilities
