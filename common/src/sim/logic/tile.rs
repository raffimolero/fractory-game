use super::orientation::{Orient, Transform};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Index, IndexMut, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Tile {
    pub id: usize,
    pub orient: Orient,
}

impl Tile {
    pub const SPACE: Self = Self {
        id: 0,
        orient: Orient::Iso,
    };

    pub fn transform(&mut self, tf: Transform) {
        self.orient.transform(tf);
    }

    pub const fn transformed(self, tf: Transform) -> Self {
        Self {
            orient: self.orient.transformed(tf),
            ..self
        }
    }
}

impl AddAssign<Transform> for Tile {
    fn add_assign(&mut self, rhs: Transform) {
        self.transform(rhs);
    }
}

impl Add<Transform> for Tile {
    type Output = Self;

    fn add(self, rhs: Transform) -> Self::Output {
        self.transformed(rhs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SubTile {
    C, // Center/Core
    U, // Up
    R, // Right
    L, // Left
}

impl SubTile {
    pub const ORDER: [Self; 4] = [Self::C, Self::U, Self::R, Self::L];
    pub const QUAD: Quad<Self> = Quad(Self::ORDER);
}

impl Display for SubTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            SubTile::C => "Center",
            SubTile::U => "Top",
            SubTile::R => "Right",
            SubTile::L => "Left",
        };
        write!(f, "{name}")
    }
}

impl AddAssign<Transform> for SubTile {
    fn add_assign(&mut self, rhs: Transform) {
        use SubTile::*;
        use Transform::*;
        match (*self, rhs) {
            (C, _) => {}
            (_, KU) => {}

            (U, FU) => {}
            (U, KL | FL) => *self = L,
            (U, KR | FR) => *self = R,

            (R, FL) => {}
            (R, FU | KR) => *self = L,
            (R, FR | KL) => *self = U,

            (L, FR) => {}
            (L, FU | KL) => *self = R,
            (L, FL | KR) => *self = U,
        }
    }
}

impl Add<Transform> for SubTile {
    type Output = Self;

    fn add(mut self, rhs: Transform) -> Self::Output {
        self += rhs;
        self
    }
}

impl Sub<Transform> for SubTile {
    type Output = Self;

    fn sub(mut self, rhs: Transform) -> Self::Output {
        self += -rhs;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Quad<T>(pub [T; 4]);
impl<T> Quad<T> {
    pub fn map<U, F: FnMut(T) -> U>(self, f: F) -> Quad<U> {
        Quad(self.0.map(f))
    }
}

impl Quad<Tile> {
    pub const SPACE: Self = Self([Tile::SPACE; 4]);
}

impl<T> Index<SubTile> for Quad<T> {
    type Output = T;

    fn index(&self, index: SubTile) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl<T> IndexMut<SubTile> for Quad<T> {
    fn index_mut(&mut self, index: SubTile) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl<T: AddAssign<Transform>> AddAssign<Transform> for Quad<T> {
    fn add_assign(&mut self, rhs: Transform) {
        for child in self.0.iter_mut() {
            *child += rhs;
        }
        if rhs.reflected() {
            self.0.swap(2, 3);
        }
        self.0[1..].rotate_right(rhs.rotation() as usize);
    }
}

impl Quad<Tile> {
    pub fn is_rfu(self) -> bool {
        use SubTile::*;
        self[C].orient.is_rfu() && self[U].orient.is_rfu() && self[R] + Transform::FU == self[L]
    }

    pub fn is_rotational(self) -> bool {
        use SubTile::*;
        self[C].orient.symmetries().is_rotational()
            && self[U] + Transform::KR == self[R]
            && self[U] + Transform::KL == self[L]
    }

    /// reorients a tringle upright, and returns its original orientation.
    pub fn reorient(&mut self) -> Orient {
        use Orient::*;
        let is_rot = self.is_rotational();
        for i in 0..3 {
            if self.is_rfu() {
                return if is_rot { Iso } else { [RfU, RfL, RfR][i] };
            }
            if is_rot {
                return RtK;
            }
            *self += Transform::KR;
        }
        AKU
    }
}
