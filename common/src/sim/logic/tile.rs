use super::orientation::{Orient, Transform};
use std::ops::{Add, AddAssign, Index, IndexMut};

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

    /// TODO: DELETE, FOR TESTING
    pub const ONE: Self = Self {
        id: 1,
        orient: Orient::Iso,
    };

    // TODO: FOR TESTING
    pub const XYYY: Self = Self {
        id: 1,
        orient: Orient::Iso,
    };
    pub const YXXX: Self = Self {
        id: 2,
        orient: Orient::Iso,
    };
}

impl AddAssign<Transform> for Tile {
    fn add_assign(&mut self, rhs: Transform) {
        self.orient += rhs;
    }
}

impl Add<Transform> for Tile {
    type Output = Self;

    fn add(mut self, rhs: Transform) -> Self::Output {
        self += rhs;
        self
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
    pub const QUAD: Quad<Self> = Quad([Self::C, Self::U, Self::R, Self::L]);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Quad<T>(pub [T; 4]);

impl Quad<Tile> {
    pub const SPACE: Self = Self([Tile::SPACE; 4]);

    // TODO: FOR TESTING
    pub const ONE: Self = Self([Tile::ONE; 4]);

    // TODO: FOR TESTING
    pub const XYYY: Self = Self([Tile::XYYY, Tile::YXXX, Tile::YXXX, Tile::YXXX]);
    pub const YXXX: Self = Self([Tile::YXXX, Tile::XYYY, Tile::XYYY, Tile::XYYY]);
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
