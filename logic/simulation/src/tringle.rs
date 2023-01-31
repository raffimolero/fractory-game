#[cfg(test)]
mod tests;

use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub},
};

pub type Tringle<T> = [T; 4];

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
impl Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        if self.reflected {
            write!(f, "%")?;
        }
        match self.rotation {
            Rotation::Up => {}
            Rotation::Right => write!(f, ">")?,
            Rotation::Left => write!(f, "<")?,
        }
        write!(f, ")")
    }
}

impl<T: AddAssign<Orientation>> AddAssign<Orientation> for Tringle<T> {
    fn add_assign(&mut self, rhs: Orientation) {
        for child in self.iter_mut() {
            child += rhs;
        }
        if rhs.reflected {
            self.swap(2, 3);
        }
        self[1..].rotate_right(rhs.rotation as usize);
   }
}

// trait Oriented {
//     /// mutates self and reorients it by the given value
//     fn reorient(&mut self, orientation: Orientation);
// }
// impl<T: Oriented> Oriented for Tringle<T> {
//     fn reorient(&mut self, orientation: Orientation) {
//         for child in self.iter_mut() {
//             child.reorient(orientation);
//         }
//         if orientation.reflected {
//             self.swap(2, 3);
//         }
//         self[1..].rotate_right(orientation.rotation as usize);
//     }
// }

