use super::*;

#[test]
fn test_create_at_root() {
    let tree = QuadTree::create_at(TilePos::UNIT, 1);
    assert_eq!(tree, QuadTree::Leaf(1));
}
