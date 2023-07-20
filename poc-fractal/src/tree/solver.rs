#[cfg(test)]
mod tests;

use ::fractory_common::sim::logic::{path::TilePos, tile::Quad};

use super::*;

#[derive(Debug, Clone, Copy)]
pub enum SetErr {
    EncounteredBad,
    EncounteredLeaf,
    StoppedAtParent,
}

// TODO: double check all pub visibilities

impl<T> Node<T> {
    pub fn create_at(mut path: TilePos, value: T) -> Self {
        match path.pop_front() {
            Some(subtile) => {
                let mut children = Quad([Node::Free, Node::Free, Node::Free, Node::Free]);
                children[subtile] = Self::create_at(path, value);
                Self::Branch(Box::new(children))
            }
            None => Self::Leaf(value),
        }
    }

    pub fn set(&mut self, mut path: TilePos, value: T) -> Result<(), SetErr> {
        // TODO: set self to bad on error
        // should i return the error?
        match self {
            Node::Bad => Err(SetErr::EncounteredBad),
            Node::Leaf(_) => Err(SetErr::EncounteredLeaf),
            Node::Free => {
                *self = Self::create_at(path, value);
                Ok(())
            }
            Node::Branch(children) => match path.pop_front() {
                Some(subtile) => children[subtile].set(path, value),
                None => Err(SetErr::StoppedAtParent),
            },
        }
    }
}
