use super::*;
use fractory_common::sim::logic::path::SubTile::{self, *};

// TODO: more tests

#[test]
fn test_create_at_root() {
    let tree = Node::create_at(TilePos::UNIT, 1);
    assert_eq!(tree, tree!(1));
}

#[test]
fn test_create_at() {
    let path = TilePos::from_inward_path(&[C, U, R, L]);
    let tree = Node::create_at(path, 4);
    assert_eq!(tree, tree!({{ . { . . { . . . 4 } . } . . } . . . }));
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
        tree!({ { . { . . { 1 . . 0 } . } { . 2 . . } . } . . .}),
    );
}

#[test]
fn test_node_set_overlapping() {
    let mut node = Node::default();

    let path_a = TilePos::from_inward_path(&[C, U, R, L]);
    let path_b = TilePos::from_inward_path(&[C, U, R, L]);

    node.set(path_a, 0).unwrap();
    node.set(path_b, 0).unwrap_err();

    assert_eq!(node, tree!({ { . { . . { . . . X } . } . . } . . . }));
}
