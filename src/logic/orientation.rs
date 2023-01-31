use std::ops::Add;

#[test]
fn test_table() {
    println!("[");
    for a in Orientation::ORIENTATIONS {
        for b in Transform::TRANSFORMS {
            let result = a.reorient(b);
            assert_eq!(result, a + b);
            println!("    Self::{result:?},");
        }
    }
    println!("]");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Orientation {
    // isotropic
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

impl Orientation {
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

    pub const fn reorient(mut self, rhs: Transform) -> Self {
        use Transform::*;
        let (flip, rot) = match rhs {
            KU => (false, 0),
            FU => (true, 0),
            KR => (false, 1),
            FR => (true, 1),
            KL => (false, 2),
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
        use Orientation::*;
        match self {
            Iso => self,
            RtK => RtF,
            RtF => RtK,
            RfU => self,
            RfR => self,
            RfL => self,
            AKU => AFU,
            AKR => AFR,
            AKL => AFL,
            AFU => AKU,
            AFR => AKR,
            AFL => AKL,
        }
    }

    pub const fn rot_cw(self) -> Self {
        use Orientation::*;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Transform {
    KU,
    FU,
    KR,
    FR,
    KL,
    FL,
}

impl Transform {
    pub const TRANSFORMS: [Self; 6] = [Self::KU, Self::FU, Self::KR, Self::FR, Self::KL, Self::FL];

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

impl Add<Transform> for Orientation {
    type Output = Self;

    fn add(self, rhs: Transform) -> Self::Output {
        [
            Self::Iso,
            Self::Iso,
            Self::Iso,
            Self::Iso,
            Self::Iso,
            Self::Iso,
            Self::RtK,
            Self::RtF,
            Self::RtK,
            Self::RtF,
            Self::RtK,
            Self::RtF,
            Self::RtF,
            Self::RtK,
            Self::RtF,
            Self::RtK,
            Self::RtF,
            Self::RtK,
            Self::RfU,
            Self::RfU,
            Self::RfR,
            Self::RfR,
            Self::RfL,
            Self::RfL,
            Self::RfR,
            Self::RfR,
            Self::RfL,
            Self::RfL,
            Self::RfU,
            Self::RfU,
            Self::RfL,
            Self::RfL,
            Self::RfU,
            Self::RfU,
            Self::RfR,
            Self::RfR,
            Self::AKU,
            Self::AFU,
            Self::AKR,
            Self::AFR,
            Self::AKL,
            Self::AFL,
            Self::AKR,
            Self::AFR,
            Self::AKL,
            Self::AFL,
            Self::AKU,
            Self::AFU,
            Self::AKL,
            Self::AFL,
            Self::AKU,
            Self::AFU,
            Self::AKR,
            Self::AFR,
            Self::AFU,
            Self::AKU,
            Self::AFR,
            Self::AKR,
            Self::AFL,
            Self::AKL,
            Self::AFR,
            Self::AKR,
            Self::AFL,
            Self::AKL,
            Self::AFU,
            Self::AKU,
            Self::AFL,
            Self::AKL,
            Self::AFU,
            Self::AKU,
            Self::AFR,
            Self::AKR,
        ][self as usize * 6 + rhs as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Rotation {
    #[default]
    U = 0,
    R = 1,
    L = 2,
}
