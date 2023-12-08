use super::tile::SubTile;
use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

#[test]
fn test_reorient_table() {
    for a in Orient::ORIENTATIONS {
        for b in Transform::TRANSFORMS {
            let result = a.reorient(b);
            // print!("{result:?}, ");
            assert_eq!(result, a + b);
        }
        // println!();
    }
}

#[test]
fn test_neg_table() {
    for a in Orient::ORIENTATIONS {
        // for b in Transform::TRANSFORMS {
        //     if a + b == a.upright() {
        //         print!("{b:?}, ");
        //         break;
        //     }
        // }
        assert_eq!(a.upright(), a - a.to_transform());
    }
    println!();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Rotation {
    #[default]
    U,
    R,
    L,
}

/// glorified bitflags
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Symmetries {
    #[default]
    Isotropic = 0b_00,
    Rotational = 0b_01,
    Reflective = 0b_10,
    Asymmetric = 0b_11,
}

impl Symmetries {
    pub const fn new(reflective: bool, rotational: bool) -> Self {
        use Symmetries::*;
        match (reflective, rotational) {
            (true, true) => Isotropic,
            (true, false) => Rotational,
            (false, true) => Reflective,
            (false, false) => Asymmetric,
        }
    }

    pub const fn is_reflective(self) -> bool {
        self as u8 & 0b_01 == 0
    }

    pub const fn is_rotational(self) -> bool {
        self as u8 & 0b_10 == 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Orient {
    // isotropic
    #[default]
    Iso,

    // rotational
    RtK, // Keep
    RtF, // Flip

    // reflective
    RfU, // Up
    RfR, // Right
    RfL, // Left

    // asymmetric
    AKU,
    AKR,
    AKL,
    AFU,
    AFR,
    AFL,
}

impl Orient {
    pub const ORIENTATIONS: [Self; 12] = [
        Self::Iso,
        Self::RtK,
        Self::RtF,
        Self::RfU,
        Self::RfR,
        Self::RfL,
        Self::AKU,
        Self::AKR,
        Self::AKL,
        Self::AFU,
        Self::AFR,
        Self::AFL,
    ];

    pub fn to_transform(self) -> Transform {
        Transform::from(self)
    }

    pub const fn transformed(self, tf: Transform) -> Self {
        use Orient::*;

        const TABLE: [Orient; 12 * 6] = [
            /*
            KU   KR   KL   FU   FR   FL   */
            Iso, Iso, Iso, Iso, Iso, Iso, //
            RtK, RtK, RtK, RtF, RtF, RtF, //
            RtF, RtF, RtF, RtK, RtK, RtK, //
            RfU, RfR, RfL, RfU, RfR, RfL, //
            RfR, RfL, RfU, RfL, RfU, RfR, //
            RfL, RfU, RfR, RfR, RfL, RfU, //
            AKU, AKR, AKL, AFU, AFR, AFL, //
            AKR, AKL, AKU, AFL, AFU, AFR, //
            AKL, AKU, AKR, AFR, AFL, AFU, //
            AFU, AFR, AFL, AKU, AKR, AKL, //
            AFR, AFL, AFU, AKL, AKU, AKR, //
            AFL, AFU, AFR, AKR, AKL, AKU, //
        ];

        TABLE[self as usize * 6 + tf as usize]
    }

    pub fn transform(&mut self, tf: Transform) {
        *self = self.transformed(tf);
    }

    pub const fn symmetries(self) -> Symmetries {
        use Orient::*;
        use Symmetries::*;
        match self {
            Iso => Isotropic,
            RtK => Rotational,
            RtF => Rotational,
            RfU => Reflective,
            RfR => Reflective,
            RfL => Reflective,
            AKU => Asymmetric,
            AKR => Asymmetric,
            AKL => Asymmetric,
            AFU => Asymmetric,
            AFR => Asymmetric,
            AFL => Asymmetric,
        }
    }

    pub const fn upright(self) -> Self {
        use Orient::*;
        use Symmetries::*;
        match self.symmetries() {
            Isotropic => Iso,
            Rotational => RtK,
            Reflective => RfU,
            Asymmetric => AKU,
        }
    }

    /// self == self.flip()
    pub const fn is_rfu(self) -> bool {
        use Orient::*;
        matches!(self, Iso | RfU)
    }

    pub const fn is_upright(self) -> bool {
        use Orient::*;
        matches!(self, Iso | RtK | RfU | AKU)
    }

    pub const fn reorient(mut self, rhs: Transform) -> Self {
        use Transform::*;
        let (flip, rot) = match rhs {
            KU => (false, 0),
            KR => (false, 1),
            KL => (false, 2),
            FU => (true, 0),
            FR => (true, 1),
            FL => (true, 2),
        };
        if flip {
            self = self.flip();
        }
        let mut i = 0;
        while i < rot {
            self = self.rot_cw();
            i += 1;
        }
        self
    }

    pub const fn flip(self) -> Self {
        use Orient::*;
        match self {
            Iso => self,
            RtK => RtF,
            RtF => RtK,
            RfU => self,
            RfR => RfL,
            RfL => RfR,
            AKU => AFU,
            AKR => AFL,
            AKL => AFR,
            AFU => AKU,
            AFR => AKL,
            AFL => AKR,
        }
    }

    pub const fn rot_cw(self) -> Self {
        use Orient::*;
        match self {
            Iso => self,
            RtK => self,
            RtF => self,
            RfU => RfR,
            RfR => RfL,
            RfL => RfU,
            AKU => AKR,
            AKR => AKL,
            AKL => AKU,
            AFU => AFR,
            AFR => AFL,
            AFL => AFU,
        }
    }
}

impl From<Symmetries> for Orient {
    fn from(symmetries: Symmetries) -> Self {
        use Orient::*;
        use Symmetries::*;
        match symmetries {
            Isotropic => Iso,
            Rotational => RtK,
            Reflective => RfU,
            Asymmetric => AKU,
        }
    }
}

impl Add<Transform> for Orient {
    type Output = Self;

    fn add(self, rhs: Transform) -> Self::Output {
        self.transformed(rhs)
    }
}

impl AddAssign<Transform> for Orient {
    fn add_assign(&mut self, rhs: Transform) {
        self.transform(rhs)
    }
}

impl Sub<Transform> for Orient {
    type Output = Self;

    fn sub(self, rhs: Transform) -> Self::Output {
        self + -rhs
    }
}

impl SubAssign<Transform> for Orient {
    fn sub_assign(&mut self, rhs: Transform) {
        *self = *self - rhs;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Transform {
    KU,
    KR,
    KL,
    FU,
    FR,
    FL,
}

impl Transform {
    pub const TRANSFORMS: [Self; 6] = [Self::KU, Self::KR, Self::KL, Self::FU, Self::FR, Self::FL];

    pub const fn reflected(self) -> bool {
        use Transform::*;
        match self {
            KU | KR | KL => false,
            FU | FR | FL => true,
        }
    }

    pub const fn rotation(self) -> Rotation {
        use Rotation::*;
        use Transform::*;
        match self {
            KU | FU => U,
            KR | FR => R,
            KL | FL => L,
        }
    }
}

impl Add for Transform {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        use Transform::*;

        const TABLE: [Transform; 6 * 6] = [
            KU, KR, KL, FU, FR, FL, //
            KR, KL, KU, FL, FU, FR, //
            KL, KU, KR, FR, FL, FU, //
            FU, FR, FL, KU, KR, KL, //
            FR, FL, FU, KL, KU, KR, //
            FL, FU, FR, KR, KL, KU, //
        ];

        TABLE[self as usize * 6 + rhs as usize]
    }
}

impl Sub for Transform {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Mul for Transform {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        -self + rhs + self
    }
}

impl Neg for Transform {
    type Output = Self;

    fn neg(self) -> Self::Output {
        use Transform::*;
        match self {
            KU => KU,
            KR => KL,
            KL => KR,
            FU => FU,
            FR => FR,
            FL => FL,
        }
    }
}

impl From<Orient> for Transform {
    fn from(value: Orient) -> Self {
        use Orient::*;
        use Transform::*;
        match value {
            Iso => KU,
            RtK => KU,
            RtF => FU,
            RfU => KU,
            RfR => KR,
            RfL => KL,
            AKU => KU,
            AKR => KR,
            AKL => KL,
            AFU => FU,
            AFR => FR,
            AFL => FL,
        }
    }
}
