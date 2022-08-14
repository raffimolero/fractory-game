//! "if you have to test your tests, then you're doing something wrong"
//!
//! looks like i aint doin anythin right

use super::*;
use std::{array, fmt::Display};

/// 3 numbers arranged [top, right, left] that
/// correspond to a specific triangle orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct TriPoint([usize; 3]);
impl Default for TriPoint {
    fn default() -> Self {
        Self(array::from_fn(|i| i))
    }
}
impl From<Orientation> for TriPoint {
    fn from(orientation: Orientation) -> Self {
        let mut out = Self::default();
        out.reorient(orientation);
        out
    }
}
impl Display for TriPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Oriented for TriPoint {
    fn reorient(
        &mut self,
        Orientation {
            reflected,
            rotation,
        }: Orientation,
    ) {
        if reflected {
            self.0.swap(1, 2);
        }
        self.0.rotate_right(rotation as usize);
    }
}

fn combinations<A: Clone, B, AIter: IntoIterator<Item = A>, BIter: IntoIterator<Item = B>>(
    a: impl FnOnce() -> AIter,
    mut b: impl FnMut() -> BIter,
) -> impl Iterator<Item = (A, B)> {
    a().into_iter()
        .flat_map(move |a| b().into_iter().map(move |b| (a.clone(), b)))
}

fn all_orientations() -> impl Iterator<Item = Orientation> {
    use Rotation::*;
    combinations(|| [false, true], || [Up, Right, Left]).map(|(reflected, rotation)| Orientation {
        reflected,
        rotation,
    })
}

#[test]
fn test_add_orientations() {
    for (a, b) in combinations(all_orientations, all_orientations) {
        let mut value = TriPoint::default();
        value.reorient(a);
        value.reorient(b);
        let sum = all_orientations()
            .find(|&candidate| TriPoint::from(candidate) == value)
            .unwrap();
        assert_eq!(a + b, sum);
    }
}

#[test]
fn test_orientation() {
    for operand in all_orientations() {
        let value = TriPoint::from(operand);
        let neg = all_orientations()
            .find(|&candidate| {
                let mut val = value.clone();
                val.reorient(candidate);
                val == TriPoint::default()
            })
            .unwrap();
        assert_eq!(-operand, neg);
        assert_eq!(operand + -operand, Orientation::default());
    }
}
