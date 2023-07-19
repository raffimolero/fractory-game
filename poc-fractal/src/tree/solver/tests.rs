use super::*;
use fractory_common::sim::logic::path::SubTile;

// TODO: more tests

#[test]
fn test_create_at_root() {
    let tree = QuadTree::create_at(TilePos::UNIT, 1);
    assert_eq!(tree, QuadTree::Leaf(1));
}

#[test]
fn test_create_at() {
    let mut path = TilePos::UNIT;
    // paths are pushed outwards, from innermost
    path.push(SubTile::L);
    path.push(SubTile::R);
    path.push(SubTile::U);
    path.push(SubTile::C);
    let tree = QuadTree::create_at(path, 4);
    assert_eq!(
        tree,
        tree! {
            { { . { . . { . . . 4 } . } . . } . . . }
        }
    );
}
