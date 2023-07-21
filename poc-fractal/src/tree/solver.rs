#[cfg(test)]
mod tests;

use std::iter::repeat_with;

use ::rand::distributions::Uniform;
use fractory_common::sim::logic::{
    actions::TileAction,
    path::{SubTile, TileOffset, TilePos},
    tile::Quad,
};

use super::*;

#[derive(Debug, Clone, Copy)]
pub enum SetErr {
    EncounteredBad,
    EncounteredLeaf,
    StoppedAtParent,
}

/// temporary struct to represent a bunch of moves
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct MoveList {
    // TODO: resolve how to order "store" operations with "move" operations
    moves: Vec<(TilePos, TilePos)>,
}

impl MoveList {
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
}

// TODO: double check all pub visibilities

type LeafItem = usize; // TODO: inline later
impl Node<LeafItem> {
    pub fn create_at(mut path: TilePos, value: LeafItem) -> Self {
        match path.pop_front() {
            Some(subtile) => {
                let mut children = Quad([Node::Free, Node::Free, Node::Free, Node::Free]);
                children[subtile] = Self::create_at(path, value);
                Self::Branch(Box::new(children))
            }
            None => Self::Leaf(value),
        }
    }

    /// returns false if a collision happened, else true.
    pub fn set(&mut self, mut path: TilePos, value: LeafItem) -> bool {
        // TODO: set self to bad on error
        // should i return the error?
        // do a direct item removal operation on the original moves vector?
        // simply consume the entire vector/iterator and turn Nodes themselves into iterators?
        // what if all 4 children are "Bad"? do we merge them into one?
        let is_ok = match self {
            Node::Bad => false,
            Node::Leaf(_) => false,
            Node::Free => {
                *self = Self::create_at(path, value);
                true
            }
            Node::Branch(children) => match path.pop_front() {
                Some(subtile) => children[subtile].set(path, value),
                None => false,
            },
        };

        if !is_ok {
            *self = Node::Bad;
        }
        is_ok
    }
}
