#[cfg(test)]
mod tests;

use ::fractory_common::sim::logic::{path::TilePos, tile::Quad};

use super::*;

#[derive(Debug, Clone, Copy)]
pub enum SetErr {
    EncounteredId,
    StoppedAtParent,
}

// TODO: double check all pub visibilities

impl<T> Node<T> {
    pub fn create_at(path: TilePos, value: T) -> Self {
        Self(Some(Box::new(QuadTree::create_at(path, value))))
    }

    pub fn set(&mut self, path: TilePos, value: T) -> Result<(), SetErr> {
        match &mut self.0 {
            Some(tree) => tree.set(path, value),
            None => {
                *self = Self::create_at(path, value);
                Ok(())
            }
        }
    }
}

impl<T> QuadTree<T> {
    pub fn create_at(mut path: TilePos, value: T) -> Self {
        match path.pop() {
            Some(subtile) => {
                let mut children = [Node(None), Node(None), Node(None), Node(None)];
                children[subtile as usize] = Node::create_at(path, value);
                Self::Branch(Quad(children))
            }
            None => Self::Leaf(value),
        }
    }

    pub fn set(&mut self, mut path: TilePos, value: T) -> Result<(), SetErr> {
        match self {
            QuadTree::Leaf(_) => Err(SetErr::EncounteredId),
            QuadTree::Branch(Quad(children)) => match path.pop() {
                Some(subtile) => children[subtile as usize].set(path, value),
                None => Err(SetErr::StoppedAtParent),
            },
        }
    }
}
