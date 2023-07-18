use ::fractory_common::sim::logic::path::TilePos;
use ::std::{
    array,
    fmt::{Debug, Display},
};
use fractory_common::sim::logic::tile::Quad;

use ::ergoquad_2d::prelude::*;
use ::rand::{distributions::Standard, prelude::*};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro() {
        let tre = tree! {
            { 1, 2, 3, { 4, 5, ., 7} }
        };
        println!("{tre}");
    }

    #[test]
    fn test_rand() {
        let mut rng = thread_rng();
        let tre = QuadTree::<u8>::rand(&mut rng, 5);
        println!("{tre}");
    }
}

// struct Node<T> = Option<Box<QuadTree<T>>>;

#[derive(Clone)]
pub enum QuadTree<T> {
    Leaf(T),
    // Branch(Quad<Node<T>>),
    Branch(Quad<Option<Box<Self>>>),
}

impl<T: Default> Default for QuadTree<T> {
    fn default() -> Self {
        Self::Leaf(T::default())
    }
}

impl<T> QuadTree<T> {
    pub fn new() -> Self
    where
        T: Default,
    {
        Self::default()
    }

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
                (!is_none).then(|| Box::new(Self::rand(rng, depth - 1)))
            })))
        }
    }

    // TODO: use a node reference
    // pub fn set(this: &mut Option<Self>, path: TilePos, value: T) {
    //     dbg!(path);
    //     for subtile in path {
    //         match this {
    //             QuadTree::Leaf(_) => todo!(),
    //             QuadTree::Branch(_) => todo!(),
    //         }
    //     }
    //     todo!()
    // }
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
                if let Some(node) = node {
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
                for child in children {
                    match child {
                        Some(val) => write!(f, "{val}")?,
                        None => write!(f, ".")?,
                    }
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
                for child in children {
                    match child {
                        Some(val) => write!(f, "{val:?}")?,
                        None => write!(f, ".")?,
                    }
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
///     { . , (), (), .  },
///     { (), (), . , () },
///     { },
///     .,
/// });
/// println!("{tree:?}");
/// ````
macro_rules! tree {
    (node .) => {
        None
    };
    (node $t:tt) => {
        Some(Box::new(tree!($t)))
    };

    ({ $tl:tt,  $tr:tt, $bl:tt, $br:tt $(,)? }) => {
        QuadTree::Branch([
            tree!(node $tl),
            tree!(node $tr),
            tree!(node $bl),
            tree!(node $br),
        ])
    };
    ($t:expr) => {
        QuadTree::Leaf($t)
    };
}
pub(crate) use tree;
