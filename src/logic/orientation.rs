use std::ops::{Add, Neg};

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
        //     if a + b == a.canon() {
        //         print!("{b:?}, ");
        //         break;
        //     }
        // }
        assert_eq!(a.canon(), a + -a);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Symmetries {
    #[default]
    Isotropic,
    Rotational,
    Reflective,
    Asymmetric,
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

    pub const fn canon(self) -> Self {
        use Orient::*;
        use Symmetries::*;
        match self.symmetries() {
            Isotropic => Iso,
            Rotational => RtK,
            Reflective => RfU,
            Asymmetric => AKU,
        }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

impl Neg for Orient {
    type Output = Transform;

    fn neg(self) -> Self::Output {
        use Orient::*;
        use Transform::*;
        match self {
            Iso => KU,
            RtK => KU,
            RtF => FU,
            RfU => KU,
            RfR => KL,
            RfL => KR,
            AKU => KU,
            AKR => KL,
            AKL => KR,
            AFU => FU,
            AFR => FR,
            AFL => FL,
        }
    }
}

impl Add<Transform> for Orient {
    type Output = Self;

    fn add(self, rhs: Transform) -> Self::Output {
        use Orient::*;

        #[rustfmt::skip]
        const TABLE: [Orient; 12 * 6] = [
        //  KU   KR   KL   FR   FU  FL
            Iso, Iso, Iso, Iso, Iso, Iso, 
            RtK, RtK, RtK, RtF, RtF, RtF, 
            RtF, RtF, RtF, RtK, RtK, RtK, 
            RfU, RfR, RfL, RfU, RfR, RfL, 
            RfR, RfL, RfU, RfL, RfU, RfR, 
            RfL, RfU, RfR, RfR, RfL, RfU, 
            AKU, AKR, AKL, AFU, AFR, AFL, 
            AKR, AKL, AKU, AFL, AFU, AFR, 
            AKL, AKU, AKR, AFR, AFL, AFU, 
            AFU, AFR, AFL, AKU, AKR, AKL, 
            AFR, AFL, AFU, AKL, AKU, AKR, 
            AFL, AFU, AFR, AKR, AKL, AKU,
        ];

        TABLE[self as usize * 6 + rhs as usize]
    }
}
