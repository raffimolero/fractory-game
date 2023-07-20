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
    let path = TilePos::from_inward_path(&[C, U, R, L]);
    let tree = QuadTree::create_at(path, 4);
    assert_eq!(
        tree,
        tree! {
            { { . { . . { . . . 4 } . } . . } . . . }
        }
    );
}

#[test]
fn test_node_set() {
    let paths: &[&[SubTile]] = &[&[C, U, R, L], &[C, U, R, C], &[C, R, U]];

    let mut node = Node::default();
    for (i, path) in paths.iter().enumerate() {
        let path = TilePos::from_inward_path(path);
        node.set(path, i).unwrap();
    }

    assert_eq!(
        node,
        Node::from(tree! {
            { { . { . . { 1 . . 0 } . } { . 2 . . } . } . . . }
        }),
    );
}
