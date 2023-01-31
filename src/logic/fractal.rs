use super::orientation::{Orient, Transform};
use std::{
    collections::HashMap,
    ops::{AddAssign, Index},
};

use indexmap::IndexSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Tile {
    id: usize,
    orient: Orient,
}

impl Tile {
    /// takes a set of 4 tiles and reorients them.
    fn reorient(tile: [Tile; 4]) -> ([Tile; 4], Orient) {
        // make sure that rotated tiles and flipped tiles are recognized correctly.
        // the center tile is the main dependency.
        // reflective tiles with isotropic centers are the only big issue.
        use Orient::*;
        match tile[0].orient {
            Iso => todo!(),
            RtK | RtF => todo!(),
            RfU => todo!(),
            RfR => todo!(),
            RfL => todo!(),
            orient @ (AKU | AKR | AKL | AFU | AFR | AFL) => (tile + -orient, orient),
        }
    }
}

struct Fractory {
    recognizer: HashMap<[Tile; 4], Tile>,
    library: Vec<[Tile; 4]>,
    root: Tile,
    // TODO: behavior: Vec<Behavior>,
}

impl Fractory {
    fn identify(&self, tile: &[Tile; 4]) -> Option<&Tile> {
        self.recognizer.get(tile)
    }

    fn register(&mut self, tile: [Tile; 4]) -> &Tile {
        if let Some(tile) = self.identify(&tile) {
            tile
        } else {
            self.register_unchecked(tile)
        }
    }

    fn register_unchecked(&mut self, tile: [Tile; 4]) -> &Tile {
        todo!("reorient tile, register tile in library, and cache it in the recognizer.");
        // let mut tile = Tile::reorient(tile);
        // self.library.push(tile);
        // for tf in Transform::TRANSFORMS {
        //     self.recognizer
        // }
    }
}

impl<T: AddAssign<Transform>> AddAssign<Transform> for [T; 4] {
    fn add_assign(&mut self, rhs: Transform) {
        for child in self.iter_mut() {
            *child += rhs;
        }
        if rhs.reflected() {
            self.swap(2, 3);
        }
        self[1..].rotate_right(rhs.rotation() as usize);
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
