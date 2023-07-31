#[cfg(test)]
mod tests;

use std::{collections::HashSet, iter::repeat_with};

use ::rand::distributions::Uniform;
use fractory_common::sim::logic::{
    actions::TileAction,
    fractal::Fractal,
    path::{SubTile, TileOffset, TilePos},
    tile::Quad,
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
        let mut tree = CollisionCleaner::default();
        let mut holes = vec![];
        for (i, (src, _dst)) in self.moves.iter().copied().enumerate() {
            tree.set(src, i, &mut |idx| holes.push(idx));
        }
        for idx in holes.iter().rev() {
            self.moves.swap_remove(*idx);
        }
    }

    fn clean_merges(&mut self) {
        let mut tree = CollisionCleaner::default();
        let mut holes = vec![];
        for (i, (_src, dst)) in self.moves.iter().copied().enumerate() {
            tree.set(dst, i, &mut |idx| holes.push(idx));
        }
        for idx in holes.iter().rev() {
            self.moves.swap_remove(*idx);
        }
    }

    fn clean_dead_ends(&mut self, tree: &Fractal) {
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
        */
        let mut dsts = CollisionCleaner::default();
        let mut srcs = CollisionCleaner::default();
        let mut ends = HashSet::new();

        for (src, dst) in self.moves.iter().copied() {
            ends.insert(dst);
            dsts.set(dst, src, &mut |_| unreachable!("destination overlaps"));
            srcs.set(src, dst, &mut |_| unreachable!("source overlaps"));
        }

        for (src, dst) in self.moves.iter().copied() {
            // dst
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct CleanMoveList {
    inner: RawMoveList,
}

// TODO: double check all pub visibilities
// TODO: figure out if you can merge Fractal with Node

impl<T> CollisionCleaner<T> {
    pub fn create_at(mut path: TilePos, value: T) -> Self {
        match path.pop_front() {
            Some(subtile) => {
                // Node does not implement Copy, hardcoding 4 frees is easier.
                let mut children = Quad([
                    CollisionCleaner::Free,
                    CollisionCleaner::Free,
                    CollisionCleaner::Free,
                    CollisionCleaner::Free,
                ]);
                children[subtile] = Self::create_at(path, value);
                Self::Branch(Box::new(children))
            }
            None => Self::Leaf(value),
        }
    }

    /// a workaround for Drop which allows mutating a shared data structure
    fn drop_with(&mut self, drop_item: &mut impl FnMut(T)) {
        match std::mem::replace(self, CollisionCleaner::Bad) {
            CollisionCleaner::Free => {}
            CollisionCleaner::Bad => {}
            CollisionCleaner::Leaf(item) => drop_item(item),
            CollisionCleaner::Branch(children) => {
                for mut node in children.0 {
                    node.drop_with(drop_item);
                }
            }
        }
    }
}

type Index = usize;
impl CollisionCleaner<Index> {
    /// sets a specified value at a specified path.
    /// calls drop_item if a collision happens.
    pub fn set(&mut self, mut path: TilePos, value: Index, drop_item: &mut impl FnMut(Index)) {
        let mut reject = |this: &mut Self| {
            drop_item(value);
            this.drop_with(drop_item);
        };

        match self {
            CollisionCleaner::Free => *self = Self::create_at(path, value),
            CollisionCleaner::Bad | CollisionCleaner::Leaf(_) => reject(self),
            CollisionCleaner::Branch(children) => match path.pop_front() {
                Some(subtile) => children[subtile].set(path, value, drop_item),
                None => reject(self),
            },
        }
    }
}

// TODO: figure out where to put CollisionCleaner and DeadEndCleaner

#[derive(Clone, PartialEq, Eq)]
pub enum DeadEndCleaner<T> {
    Leaf(T),
    Branch(Box<Quad<Self>>),
}

impl<T> DeadEndCleaner<T> {
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
            DeadEndCleaner::Leaf(val) => {
                draw_base();
                draw_leaf(val);
            }
            DeadEndCleaner::Branch(children) => {
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

impl<T: Display> Display for DeadEndCleaner<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeadEndCleaner::Leaf(val) => val.fmt(f),
            DeadEndCleaner::Branch(children) => {
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
impl DeadEndCleaner<Path> {
    pub fn free_children(&mut self) {}

    pub fn mark_src(&mut self, mut src: Path) {
        match self {
            DeadEndCleaner::Leaf(existing_src) => panic!(),
            DeadEndCleaner::Branch(children) => match src.pop_front() {
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
            DeadEndCleaner::Free => *self = Self::create_at(dst, src),
            DeadEndCleaner::Bad => panic!(),
            DeadEndCleaner::Leaf(existing_dst) => panic!(),
            DeadEndCleaner::Branch(children) => match dst.pop_front() {
                Some(subtile) => children[subtile].set_dst(src, dst),
                None => panic!(),
            },
        }
    }
}
