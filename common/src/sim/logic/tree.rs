mod collision;

use super::{path::TilePos, tile::Quad};
use std::fmt::{Debug, Display};

type Index = usize;

// TODO: figure out if you can merge Fractal with CollisionCleaner (Node) and DeadEndCleaner (hidden in collision::clean_dead_ends)

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
            Node::Branch(children) => match path.pop_front() {
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
/// ```
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
