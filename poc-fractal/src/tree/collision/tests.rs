use super::*;
use fractory_common::sim::logic::path::SubTile::{self, *};

#[test]
fn test_macro() {
    let tree = tree! {
        {
            1
            3
            {
                { // this block is an expression, not a branch
                    let x = 5;
                    let y = 6;
                    x + y
                }
                .
                X
                .
            } {
                7
                9
                .
                11
            }
        }
    };
    assert_eq!(
        tree,
        Node::Branch(Box::new(Quad([
            Node::Leaf(1),
            Node::Leaf(3),
            Node::Branch(Box::new(Quad([
                Node::Leaf({
                    let x = 5;
                    let y = 6;
                    x + y
                }),
                Node::Free,
                Node::Bad,
                Node::Free,
            ]))),
            Node::Branch(Box::new(Quad([
                Node::Leaf(7),
                Node::Leaf(9),
                Node::Free,
                Node::Leaf(11),
            ]))),
        ]))),
    );
}

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

/// iterates through a sequence of paths, expected deletions, and tree snapshots:
/// - set the item at the path,
/// - monitor exactly which tiles it deletes through a mock delete function, and
/// - monitor the state of the tree at every step
fn mock_node_set(sequence: &[(&[SubTile], &[Index], Node)]) {
    let mut node = Node::default();

    for (i, (path, expected_deletions, expected_tree)) in sequence.iter().enumerate() {
        // build a mock deletion function that expects to be
        // called with specific arguments in a specific order
        let mut iter = expected_deletions.iter().copied();
        let mut mock_delete = |idx| assert_eq!(iter.next(), Some(idx));

        let path = TilePos::from_inward_path(path);
        node.set(path, i, &mut mock_delete);

        // must consume the whole sequence
        assert_eq!(iter.next(), None);

        assert_eq!(node, *expected_tree);
    }
}

#[test]
fn test_node_set() {
    mock_node_set(&[
        (
            &[C, U, R, L],
            &[],
            tree!({ { . { . . { . . . 0 } . } . . } . . .}),
        ),
        (
            &[C, U, R, C],
            &[],
            tree!({ { . { . . { 1 . . 0 } . } . . } . . .}),
        ),
        (
            &[C, R, U],
            &[],
            tree!({ { . { . . { 1 . . 0 } . } { . 2 . . } . } . . .}),
        ),
    ]);
}

#[test]
fn test_node_set_same_leaf() {
    mock_node_set(&[
        (
            &[C, U, R, L],
            &[],
            tree!({ { . { . . { . . . 0 } . } . . } . . . }),
        ),
        (
            &[C, U, R, L],
            &[1, 0],
            tree!({ { . { . . { . . . X } . } . . } . . . }),
        ),
    ])
}

#[test]
fn test_node_set_pass_leaf() {
    mock_node_set(&[
        (
            &[C, U, R, L],
            &[],
            tree!({ { . { . . { . . . 0 } . } . . } . . . }),
        ),
        (
            &[C, U, R, L, C],
            &[1, 0],
            tree!({ { . { . . { . . . X } . } . . } . . . }),
        ),
    ])
}

#[test]
fn test_node_set_hit_branch() {
    mock_node_set(&[
        (
            &[C, U, R, L, C],
            &[],
            tree!({ { . { . . { . . . { 0 . . . } } . } . . } . . . }),
        ),
        (
            &[C, U, R, L, U],
            &[],
            tree!({ { . { . . { . . . { 0 1 . . } } . } . . } . . . }),
        ),
        (
            &[C, U, R, L, R],
            &[],
            tree!({ { . { . . { . . . { 0 1 2 . } } . } . . } . . . }),
        ),
        (
            &[C, U, R, L, L],
            &[],
            tree!({ { . { . . { . . . { 0 1 2 3 } } . } . . } . . . }),
        ),
        (
            &[C, U, R, L],
            &[4, 0, 1, 2, 3],
            tree!({ { . { . . { . . . X } . } . . } . . . }),
        ),
    ])
}
