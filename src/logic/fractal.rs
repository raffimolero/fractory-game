use super::orientation::Transform;
use std::{
    collections::HashMap,
    ops::{AddAssign, Index},
};

use indexmap::IndexSet;

pub struct Tringle<T>([T; 4]);

impl<T: AddAssign<Transform>> AddAssign<Transform> for Tringle<T> {
    fn add_assign(&mut self, rhs: Transform) {
        for child in self.0.iter_mut() {
            *child += rhs;
        }
        if rhs.reflected() {
            self.0.swap(2, 3);
        }
        self.0[1..].rotate_right(rhs.rotation() as usize);
    }
}

#[derive(Debug)]
pub struct Fractal {
    root: usize,
    nodes: IndexSet<[usize; 4]>,
}

impl Fractal {
    pub fn new() -> Self {
        let mut nodes = IndexSet::new();
        nodes.insert([0; 4]);
        Self { root: 0, nodes }
    }

    pub fn load(root: usize, node_array: impl IntoIterator<Item = [usize; 4]>) -> Self {
        let nodes = node_array.into_iter().collect::<IndexSet<[usize; 4]>>();
        Self { root, nodes }
    }

    pub fn set(&mut self, path: impl IntoIterator<Item = usize>, node: usize) -> usize {
        // expand each child in the path
        let mut replaced_node = self.root;
        let expansions = path
            .into_iter()
            .map(|next| {
                let children = self.nodes[replaced_node];
                replaced_node = children[next];
                (children, next)
            })
            .collect::<Vec<_>>();

        if replaced_node == node {
            return node;
        }

        // backtrack each expansion, replacing the old nodes with new ones each time
        self.root = expansions
            .into_iter()
            .rev()
            .fold(node, |cur_node, (mut quad, child)| {
                quad[child] = cur_node;
                self.nodes.insert_full(quad).0
            });

        replaced_node
    }
}

impl<T: IntoIterator<Item = usize>> Index<T> for Fractal {
    type Output = usize;

    fn index(&self, path: T) -> &Self::Output {
        path.into_iter().fold(&self.root, |r, i| &self.nodes[*r][i])
    }
}

struct Tile {
    id: usize,
    orientation: Transform,
}

struct Fractory {
    recognizer: HashMap<Tringle<Tile>, Tile>,
}
