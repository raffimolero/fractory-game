use crate::tringle::{Orientation, Tringle};
use std::collections::HashMap;

/// please don't create more than 65536 well-defined fragments. that would suck.
pub struct FragmentId(u16);

pub struct FragmentLibrary {
    identifier: HashMap<[Fragment; 4], Fragment>,
}

pub struct Fragment {
    id: FragmentId,
    orientation: Orientation,
}

pub struct Slot(Tringle<Option<Fragment>>);
