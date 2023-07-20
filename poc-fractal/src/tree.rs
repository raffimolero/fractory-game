use ::fractory_common::sim::logic::tile::Quad;
use ::std::{
    array,
    fmt::{Debug, Display},
};

use ::ergoquad_2d::prelude::*;
use ::rand::{distributions::Standard, prelude::*};

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
                    .
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
            QuadTree::Branch(Quad([
                Node(Some(Box::new(QuadTree::Leaf((1, 2))))),
                Node(Some(Box::new(QuadTree::Leaf((3, 4))))),
                Node(Some(Box::new(QuadTree::Branch(Quad([
                    Node(Some(Box::new(QuadTree::Leaf({
                        let x = 5;
                        let y = 6;
                        (x, y)
                    })))),
                    Node(None),
                    Node(None),
                    Node(None),
                ]))))),
                Node(Some(Box::new(QuadTree::Branch(Quad([
                    Node(Some(Box::new(QuadTree::Leaf((7, 8))))),
                    Node(Some(Box::new(QuadTree::Leaf((9, 10))))),
                    Node(None),
                    Node(Some(Box::new(QuadTree::Leaf((11, 12))))),
                ]))))),
            ])),
        );
    }

    #[test]
    fn test_rand() {
        let mut rng = thread_rng();
        let tre = QuadTree::<u8>::rand(&mut rng, 5);
        println!("{tre}");
    }
}

#[derive(Clone, PartialEq, Eq, Default)]
pub struct Node<T>(Option<Box<QuadTree<T>>>);

impl<T> From<QuadTree<T>> for Node<T> {
    fn from(value: QuadTree<T>) -> Self {
        Self(Some(Box::new(value)))
    }
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(val) => val.fmt(f),
            None => write!(f, "."),
        }
    }
}

impl<T: Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(val) => val.fmt(f),
            None => write!(f, "."),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum QuadTree<T> {
    Leaf(T),
    Branch(Quad<Node<T>>),
}

impl<T> QuadTree<T> {
    pub fn rand(rng: &mut impl Rng, depth: usize) -> Self
    where
        Standard: Distribution<T>,
    {
        let is_leaf = rng.gen_ratio(1, 1 << depth);
        if is_leaf {
            Self::Leaf(rng.gen())
        } else {
            Self::Branch(Quad(array::from_fn(|_| {
                let is_none = rng.gen_ratio(1, 1 << depth);
                let children = (!is_none).then(|| Box::new(Self::rand(rng, depth - 1)));
                Node(children)
            })))
        }
    }
}

impl<T> QuadTree<T> {
    fn _draw(&self, draw_leaf: &impl Fn(&T), depth: usize) {
        let palette = [RED, ORANGE, YELLOW, GREEN, BLUE, PURPLE];
        let col = palette[depth % palette.len()];

        // draw base color
        draw_rectangle(0.0, 0.0, 1.0, 1.0, col);

        // draw outline
        // let outline_thickness = 1.0 / 64.0;
        // draw_rectangle_lines(0.0, 0.0, 1.0, 1.0, outline_thickness, BLACK);

        let children = match self {
            QuadTree::Leaf(val) => return draw_leaf(val),
            QuadTree::Branch(Quad(children)) => children,
        };

        // margin between child trees
        let margin = 1.0 / 16.0;

        let scale = upscale(0.5 - margin * 1.5);
        for (y, row) in children.chunks_exact(2).enumerate() {
            let y = y as f32 * (0.5 - margin / 2.0) + margin;
            for (x, node) in row.iter().enumerate() {
                let x = x as f32 * (0.5 - margin / 2.0) + margin;
                if let Node(Some(node)) = node {
                    apply(shift(x, y) * scale, || node._draw(draw_leaf, depth + 1));
                }
            }
        }
    }

    pub fn draw(&self, draw_leaf: &impl Fn(&T)) {
        self._draw(draw_leaf, 0);
    }
}

impl<T: Display> Display for QuadTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuadTree::Leaf(val) => val.fmt(f),
            QuadTree::Branch(Quad(children)) => {
                write!(f, "{{ ")?;
                for Node(child) in children {
                    child.fmt(f)?;
                    write!(f, " ")?;
                }
                write!(f, "}}")
            }
        }
    }
}

impl<T: Debug> Debug for QuadTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuadTree::Leaf(val) => val.fmt(f),
            QuadTree::Branch(Quad(children)) => {
                write!(f, "{{ ")?;
                for Node(child) in children {
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
    (node .) => {
        Node(None)
    };
    (node $t:tt) => {
        Node(Some(Box::new(tree!($t))))
    };

    ({ $tl:tt $tr:tt $bl:tt $br:tt }) => {
        QuadTree::Branch(Quad([
            tree!(node $tl),
            tree!(node $tr),
            tree!(node $bl),
            tree!(node $br),
        ]))
    };
    ($t:expr) => {
        QuadTree::Leaf($t)
    };
}
pub(crate) use tree;
