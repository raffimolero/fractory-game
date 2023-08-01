#[cfg(test)]
mod tests;

use std::{collections::HashSet, iter::repeat_with};

use ::rand::distributions::Uniform;
use fractory_common::sim::logic::{
    actions::TileAction,
    fractal::Fractal,
    path::{SubTile, TileOffset, TilePos},
    tile::{Quad, Tile},
};

use super::*;

/// temporary struct to represent a bunch of moves
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct RawMoveList {
    // TODO: resolve how to order "store" operations with "move" operations
    moves: Vec<(TilePos, TilePos)>,
}

// TODO: figure out how to make the coupling with the fractal quadtree clearer,
// because Fractal <- RawMoveList <- Node<LeafItem> and the dependence is clear

impl RawMoveList {
    fn rand(len: usize) -> Self {
        let rng = thread_rng();
        let dist = Uniform::new(0, 5);
        let mut samples = dist.sample_iter(rng);
        let mut rand_path = || {
            let mut path = TilePos::UNIT;
            loop {
                let subtile = match samples.next().unwrap() {
                    0 => SubTile::C,
                    1 => SubTile::U,
                    2 => SubTile::R,
                    3 => SubTile::L,
                    _ => return path,
                };
                path.push_front(subtile);
            }
        };

        let moves = repeat_with(|| (rand_path(), rand_path()))
            .take(len)
            .collect();

        Self { moves }
    }

    fn clean_sources(&mut self, tree: &Fractal) {
        // TODO: clean attempts to move from tile that contains space
        let mut set = HashSet::new();
        let mut i = 0;
        while let Some(item) = self.moves.get(i).copied() {
            if set.insert(item) {
                i += 1;
            } else {
                self.moves.swap_remove(i);
            }
        }
    }

    fn clean_forks(&mut self) {
        let mut tree = CollisionTree::default();
        let mut holes = vec![];
        for (i, (src, _dst)) in self.moves.iter().copied().enumerate() {
            tree.set(src, i, &mut |idx| holes.push(idx));
        }
        for idx in holes.iter().rev() {
            self.moves.swap_remove(*idx);
        }
    }

    fn clean_merges(&mut self) {
        let mut tree = CollisionTree::default();
        let mut holes = vec![];
        for (i, (_src, dst)) in self.moves.iter().copied().enumerate() {
            tree.set(dst, i, &mut |idx| holes.push(idx));
        }
        for idx in holes.iter().rev() {
            self.moves.swap_remove(*idx);
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

        // TODO: build the rest of algo
        // then test algo
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

        let mut old_tiles = vec![];
        for (i, (src, dst)) in self.moves.iter().copied().enumerate() {
            old_tiles.push(main_fractal.get(src));
            // srcs.set(src, main_fractal.get(src));
            dsts.set(dst, i);
            main_fractal.set(src, Tile::SPACE); // TODO
        }

        let mut dead = vec![];
        for (i, (src, dst)) in self.moves.iter().copied().enumerate() {
            if main_fractal.get(dst) != Tile::SPACE {
                dead.push(i);
            }
        }

        while let Some(i) = dead.pop() {
            let (src, dst) = self.moves[i];
            main_fractal.set(src, old_tiles[i]);
            dsts.invalidate(src, &mut |i| dead.push(i));
        }

        impl<T> Tree<T> {
            fn set(&mut self, pos: TilePos, val: T) {
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
            fn invalidate(&mut self, pos: TilePos, drop_item: &mut impl FnMut(Index)) {
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
                        for child in children.0 {
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
struct CleanMoveList {
    inner: RawMoveList,
}

// TODO: double check all pub visibilities

// TODO: figure out where to put CollisionTree and DeadEndTree

#[derive(Clone, PartialEq, Eq)]
pub enum DeadEndTree<T> {
    Leaf(T),
    Branch(Box<Quad<Self>>),
}

impl<T> DeadEndTree<T> {
    const PALETTE: &[Color] = &[RED, ORANGE, YELLOW, GREEN, BLUE, PURPLE];

    pub fn random_paths(rng: &mut impl Rng, path_count: usize) -> Self
    where
        Standard: Distribution<T>,
    {
        todo!()
    }

    pub fn draw(&self, draw_leaf: &impl Fn(&T)) {
        self._draw(draw_leaf, 0);
    }

    fn _draw(&self, draw_leaf: &impl Fn(&T), depth: usize) {
        let col = Self::PALETTE[depth % Self::PALETTE.len()];
        let draw_base = || {
            draw_rectangle(0.0, 0.0, 1.0, 1.0, col);

            // // draw outline
            // let outline_thickness = 1.0 / 64.0;
            // draw_rectangle_lines(0.0, 0.0, 1.0, 1.0, outline_thickness, BLACK);
        };
        match self {
            DeadEndTree::Leaf(val) => {
                draw_base();
                draw_leaf(val);
            }
            DeadEndTree::Branch(children) => {
                draw_base();

                // margin between child trees
                let margin = 1.0 / 16.0;

                let scale = upscale(0.5 - margin * 1.5);
                for (y, row) in children.0.chunks_exact(2).enumerate() {
                    let y = y as f32 * (0.5 - margin / 2.0) + margin;
                    for (x, node) in row.iter().enumerate() {
                        let x = x as f32 * (0.5 - margin / 2.0) + margin;
                        apply(shift(x, y) * scale, || node._draw(draw_leaf, depth + 1))
                    }
                }
            }
        }
    }
}

impl<T: Display> Display for DeadEndTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeadEndTree::Leaf(val) => val.fmt(f),
            DeadEndTree::Branch(children) => {
                write!(f, "{{ ")?;
                for child in &children.0 {
                    child.fmt(f)?;
                    write!(f, " ")?;
                }
                write!(f, "}}")
            }
        }
    }
}

type Path = TilePos;
impl DeadEndTree<Path> {
    pub fn free_children(&mut self) {}

    pub fn mark_src(&mut self, mut src: Path) {
        match self {
            DeadEndTree::Leaf(existing_src) => panic!(),
            DeadEndTree::Branch(children) => match src.pop_front() {
                Some(subtile) => children[subtile].mark_src(src),
                None => {
                    self.free_children();
                    todo!("hit parent, mark children as free")
                }
            },
        }
    }

    pub fn set_dst(&mut self, src: Path, mut dst: Path) {
        match self {
            DeadEndTree::Free => *self = Self::create_at(dst, src),
            DeadEndTree::Bad => panic!(),
            DeadEndTree::Leaf(existing_dst) => panic!(),
            DeadEndTree::Branch(children) => match dst.pop_front() {
                Some(subtile) => children[subtile].set_dst(src, dst),
                None => panic!(),
            },
        }
    }
}
