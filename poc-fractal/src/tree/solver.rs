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

    fn delete(&mut self, delete_item: &mut impl FnMut(LeafItem)) {
        match self {
            Node::Free => {}
            Node::Bad => {}
            Node::Leaf(item) => delete_item(*item),
            Node::Branch(children) => {
                for node in &mut children.0 {
                    node.delete(delete_item);
                }
            }
        }
        *self = Node::Bad;
    }

    /// sets a specified value at a specified path.
    /// calls delete_item if a collision happens.
    pub fn set(
        &mut self,
        mut path: TilePos,
        value: LeafItem,
        delete_item: &mut impl FnMut(LeafItem),
    ) {
        let mut reject = |this: &mut Self| {
            delete_item(value);
            this.delete(delete_item);
        };

        match self {
            Node::Free => *self = Self::create_at(path, value),
            Node::Bad | Node::Leaf(_) => reject(self),
            Node::Branch(children) => match path.pop_front() {
                Some(subtile) => children[subtile].set(path, value, delete_item),
                None => reject(self),
            },
        }
    }
}
