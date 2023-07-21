mod solver;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro() {
        let tree = tree! {
            {
                (1, 2)
                (3, 4)
                {
                    { // this block is an expression, not a branch
                        let x = 5;
                        let y = 6;
                        (x, y)
                    }
                    .
                    X
                    .
                } {
                    (7, 8)
                    (9, 10)
                    .
                    (11, 12)
                }
            }
        };
        assert_eq!(
            tree,
            Node::Branch(Box::new(Quad([
                Node::Leaf((1, 2)),
                Node::Leaf((3, 4)),
                Node::Branch(Box::new(Quad([
                    Node::Leaf({
                        let x = 5;
                        let y = 6;
                        (x, y)
                    }),
                    Node::Free,
                    Node::Bad,
                    Node::Free,
                ]))),
                Node::Branch(Box::new(Quad([
                    Node::Leaf((7, 8)),
                    Node::Leaf((9, 10)),
                    Node::Free,
                    Node::Leaf((11, 12)),
                ]))),
            ]))),
        );
    }
}

use fractory_common::sim::logic::tile::Quad;
use std::fmt::{Debug, Display};

use ::rand::{distributions::Standard, prelude::*};
use ergoquad_2d::prelude::*;

#[derive(Clone, PartialEq, Eq, Default)]
pub enum Node<T> {
    #[default]
    Free,
    Bad,
    Leaf(T),
    Branch(Box<Quad<Self>>),
}

impl<T> Node<T> {
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
            Node::Free => {}
            Node::Bad => draw_poly(0.0, 0.0, 4, 1.0, 45.0, col),
            Node::Leaf(val) => {
                draw_base();
                draw_leaf(val);
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
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Free => write!(f, " "),
            Node::Bad => write!(f, "X"),
            Node::Leaf(val) => val.fmt(f),
            Node::Branch(children) => {
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

impl<T: Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Free => write!(f, " "),
            Node::Bad => write!(f, "X"),
            Node::Leaf(val) => val.fmt(f),
            Node::Branch(children) => {
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
