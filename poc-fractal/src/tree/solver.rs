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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeResponse {
    Accept,
    Reject,
    Contradict(LeafItem),
}

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
    pub fn set(&mut self, mut path: TilePos, value: LeafItem) -> NodeResponse {
        let response = match self {
            Node::Bad => NodeResponse::Reject,
            Node::Leaf(item) => NodeResponse::Contradict(*item),
            Node::Free => {
                *self = Self::create_at(path, value);
                NodeResponse::Accept
            }
            Node::Branch(children) => match path.pop_front() {
                // NOTE: even if all 4 children are "Bad" we do not merge them into one
                // NOTE: must early return
                Some(subtile) => return children[subtile].set(path, value),
                None => NodeResponse::Reject,
            },
        };

        if !matches!(response, NodeResponse::Accept) {
            *self = Node::Bad;
        }
        response
    }
}
