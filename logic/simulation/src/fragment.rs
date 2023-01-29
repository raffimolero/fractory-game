/* use crate::{
    tree::{Expander, Node},
    tringle::{Orientation, Tringle},
};
use std::collections::HashMap;

/// please don't define more than 65536 different fragments. that would suck.
pub struct FragmentId(u16);

pub struct FragmentLibrary {
    identifier: HashMap<[Fragment; 4], Fragment>,
    expander: Vec<[Fragment; 4]>,
}
// impl Expander<Fragment, 4> for FragmentLibrary {
//     fn expand(&self, item: &Fragment) -> [Fragment; 4] {
//         let children = self.expander[item.id.0 as usize];
//     }
// }

pub struct Fragment {
    id: FragmentId,
    orientation: Orientation,
}

// pub type Slot = Node<Tringle<Option<Fragment>>>;

// #[test]
// fn test_slot() {
//     let slot = Slot::Leaf(None);
// }
 */
