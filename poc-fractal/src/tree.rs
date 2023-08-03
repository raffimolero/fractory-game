mod collision;

use fractory_common::sim::logic::{path::TilePos, tile::Quad};
use std::fmt::{Debug, Display};

use ::rand::{distributions::Standard, prelude::*};
use ergoquad_2d::prelude::*;

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
    const PALETTE: &[Color] = &[RED, ORANGE, YELLOW, GREEN, BLUE, PURPLE];

    pub fn random_paths(rng: &mut impl Rng, path_count: usize) -> Self
    where
        Standard: Distribution<Index>,
    {
        todo!()
    }

    pub fn draw(&self, draw_leaf: &impl Fn(Index)) {
        self._draw(draw_leaf, 0);
    }

    fn _draw(&self, draw_leaf: &impl Fn(Index), depth: usize) {
        let col = Self::PALETTE[depth % Self::PALETTE.len()];
        let draw_base = || {
            draw_rectangle(0.0, 0.0, 1.0, 1.0, col);

            // // draw outline
            // let outline_thickness = 1.0 / 64.0;
            // draw_rectangle_lines(0.0, 0.0, 1.0, 1.0, outline_thickness, BLACK);
        };
        match self {
            Node::Free => {}
            Node::Bad => draw_poly(0.0, 0.0, 4, 1.0, 45.0, col),
            Node::Leaf(val) => {
                draw_base();
                draw_leaf(*val);
            }
            Node::Branch(children) => {
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
