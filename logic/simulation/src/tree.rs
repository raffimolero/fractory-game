use std::collections::HashMap;

pub type NodeId = usize;

#[derive(Debug, Default)]
pub struct NodeLibrary<const N: usize> {
    idx_to_node: Vec<[NodeId; N]>,
    node_to_idx: HashMap<[NodeId; N], NodeId>,
}
impl<const N: usize> NodeLibrary<N> {
    /// creates a new node library, initialized with an "empty tile" node.
    fn new() -> Self {
        
    }
}

#[derive(Debug, Default)]
pub struct Fractal<const N: usize> {
    node_library: NodeLibrary<N>,
    root: NodeId,
}

impl<const N: usize> Fractal<N> {
    fn new() -> Self {
        Self {
            node_library: NodeLibrary {
                idx_to_node: vec![[0; N]],
                node_to_idx: HashMap::from([([0; N], 0)]),
            },
            root: 0,
        }
    }
}

/*
definitions:
    1: Sierpinski

inventory:
    1: 5

hashmap:
    0 -> [0, 0, 0, 0]
    1 -> [0, 1, 1, 1]
    2 -> [0, 1, 0, 1]
    3 -> [0, 2, 0, 2]
    root: 3

  1
  0
3   2
*/
