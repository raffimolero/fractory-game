use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Rotation {
    #[default]
    No = 0,
    Right = 1,
    Left = 2,
}

impl Add for Rotation {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        use Rotation::*;
        match (self as u8 + rhs as u8) % 3 {
            0 => No,
            1 => Right,
            2 => Left,
            _ => unreachable!(),
        }
    }
}

impl AddAssign for Rotation {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Neg for Rotation {
    type Output = Self;

    fn neg(self) -> Self::Output {
        use Rotation::*;
        match self {
            No => No,
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
impl AddAssign for Orientation {
    fn add_assign(&mut self, rhs: Self) {
        if rhs.reflected {
            self.rotation = -self.rotation;
        }
        self.reflected ^= rhs.reflected;
        self.rotation += rhs.rotation;
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

impl Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        if self.reflected {
            write!(f, "%")?;
        }
        match self.rotation {
            Rotation::No => {}
            Rotation::Right => write!(f, ">")?,
            Rotation::Left => write!(f, "<")?,
        }
        write!(f, ")")
    }
}
