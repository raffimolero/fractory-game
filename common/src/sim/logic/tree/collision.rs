#[cfg(test)]
mod tests;

use std::collections::{BTreeSet, HashSet};

use crate::sim::logic::{
    fractal::Fractal,
    orientation::Transform,
    path::TilePos,
    tile::{Quad, Tile},
};

use super::*;

/// temporary struct to represent a bunch of moves
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RawMoveList {
    moves: Vec<(TilePos, (TilePos, Transform))>,
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

        // NOTE: this is so f###ing complicated dude
        // i'm dealing with fractals right now

        // TODO: test algo
        // this is an in-place algo btw

        // hypothetical algorithm

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
            assert_ne!(old_tile, Tile::SPACE);
            assert!(main_fractal.get_info(old_tile.id).fill.is_full());
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
            let (src, _dst_tf) = self.moves[i];
            if old_tiles[i] == Tile::SPACE {
                continue;
            }
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

                let Some(subtile) = pos.pop_front() else {
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
                    Tree::Branch(children) => match pos.pop_front() {
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

// TODO: double check all pub visibilities
