use super::*;
use fractory_common::sim::logic::path::SubTile::{self, *};

// TODO: more tests

#[test]
fn test_create_at_root() {
    let tree = QuadTree::create_at(TilePos::UNIT, 1);
    assert_eq!(tree, QuadTree::Leaf(1));
}

#[test]
fn test_create_at() {
    let mut path = TilePos::from_inward_path(&[C, U, R, L]);
    let tree = QuadTree::create_at(path, 4);
    assert_eq!(
        tree,
        tree! {
            { { . { . . { . . . 4 } . } . . } . . . }
        }
    );
}
