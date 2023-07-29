#[cfg(test)]
mod tests;

use std::{collections::HashSet, iter::repeat_with};

use ::rand::distributions::Uniform;
use fractory_common::sim::logic::{
    actions::TileAction,
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

    fn dedup(&mut self) {
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
        let mut tree = Node::default();
        let mut holes = vec![];
        for (i, (src, _dst)) in self.moves.iter().copied().enumerate() {
            tree.set(src, i, &mut |idx| holes.push(idx));
        }
        for idx in holes.iter().rev() {
            self.moves.swap_remove(*idx);
        }
    }

    fn clean_merges(&mut self) {
        let mut tree = Node::default();
        let mut holes = vec![];
        for (i, (_src, dst)) in self.moves.iter().copied().enumerate() {
            tree.set(dst, i, &mut |idx| holes.push(idx));
        }
        for idx in holes.iter().rev() {
            self.moves.swap_remove(*idx);
        }
    }

    fn clean_dead_ends(&mut self) {
        
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct CleanMoveList {
    inner: RawMoveList,
}

// TODO: double check all pub visibilities

type LeafItem = usize; // TODO: inline later
impl Node<LeafItem> {
    pub fn create_at(mut path: TilePos, value: LeafItem) -> Self {
        match path.pop_front() {
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
    fn drop_with(&mut self, drop_item: &mut impl FnMut(LeafItem)) {
        match self {
            Node::Free => {}
            Node::Bad => {}
            Node::Leaf(item) => drop_item(*item),
            Node::Branch(children) => {
                for node in &mut children.0 {
                    node.drop_with(drop_item);
                }
            }
        }
        *self = Node::Bad;
    }

    /// sets a specified value at a specified path.
    /// calls drop_item if a collision happens.
    pub fn set(
        &mut self,
        mut path: TilePos,
        value: LeafItem,
        drop_item: &mut impl FnMut(LeafItem),
    ) {
        let mut reject = |this: &mut Self| {
            drop_item(value);
            this.drop_with(drop_item);
        };

        match self {
            Node::Free => *self = Self::create_at(path, value),
            Node::Bad | Node::Leaf(_) => reject(self),
            Node::Branch(children) => match path.pop_front() {
                Some(subtile) => children[subtile].set(path, value, drop_item),
                None => reject(self),
            },
        }
    }
}
