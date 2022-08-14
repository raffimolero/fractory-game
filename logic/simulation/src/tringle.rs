#[cfg(test)]
mod tests;

use crate::tree::Node;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub type Tringle<T> = Node<T, 4>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Rotation {
    #[default]
    Up = 0,
    Right = 1,
    Left = 2,
}
impl Add for Rotation {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        use Rotation::*;
        match (self as u8 + rhs as u8) % 3 {
            0 => Up,
            1 => Right,
            2 => Left,
            _ => unreachable!(),
        }
    }
}
impl Neg for Rotation {
    type Output = Self;

    fn neg(self) -> Self::Output {
        use Rotation::*;
        match self {
            Up => Up,
            Right => Left,
            Left => Right,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Orientation {
    pub reflected: bool,
    pub rotation: Rotation,
}
impl Add for Orientation {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        if rhs.reflected {
            self.rotation = -self.rotation;
        }
        Self {
            reflected: self.reflected ^ rhs.reflected,
            rotation: self.rotation + rhs.rotation,
        }
    }
}
impl Neg for Orientation {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        if !self.reflected {
            self.rotation = -self.rotation;
        }
        self
    }
}

trait Oriented {
    /// mutates self and reorients it by the given value
    fn reorient(&mut self, orientation: Orientation);
}

impl<T: Oriented> Oriented for Tringle<T> {
    fn reorient(&mut self, orientation: Orientation) {
        match self {
            Node::Leaf(item) => item.reorient(orientation),
            Node::Branch(children) => {
                for child in children.iter_mut() {
                    child.reorient(orientation);
                }
                if orientation.reflected {
                    children.swap(2, 3);
                }
                children[1..].rotate_right(orientation.rotation as usize);
            }
        }
    }
}
