#[cfg(test)]
mod tests;

use super::{
    orientation::Transform,
    quad::{QuadTile, Tile},
};
use std::{collections::HashMap, ops::Index};

use indexmap::IndexSet;

struct Fractory {
    root: Tile,
    library: Vec<QuadTile>,
    recognizer: HashMap<QuadTile, Tile>,
    // TODO: behavior: Vec<Behavior>,
}

impl Fractory {
    fn new() -> Self {
        Self {
            root: Tile::SPACE,
            library: vec![QuadTile::SPACE],
            recognizer: HashMap::from([(QuadTile::SPACE, Tile::SPACE)]),
            // behavior: vec![Behavior::NONE],
        }
    }

    // TODO: make Fragment data structure
    fn load_base(root: Tile, nodes: impl IntoIterator<Item = Fragment>) -> Self {
        todo!("get fragment data such as symmetries, composition, and behaviors")
    }

    fn register(&mut self, quad: QuadTile) -> Tile {
        self.recognizer
            .get(&quad)
            .copied()
            .unwrap_or_else(|| self.register_new(quad))
    }

    fn register_new(&mut self, mut quad: QuadTile) -> Tile {
        let orient = quad.reorient();
        let id = self.library.len();
        self.library.push(quad);

        self.cache(
            quad,
            Tile {
                id,
                orient: orient.upright(),
            },
        );

        Tile { id, orient }
    }

    fn cache(&mut self, mut quad: QuadTile, mut tile: Tile) {
        for _reflection in 0..2 {
            for _rotation in 0..3 {
                let result = self.recognizer.insert(quad, tile);
                assert!(
                    result.is_none(),
                    "Tringle ({quad:?}) was registered ({tile:?}) twice!"
                );

                if tile.orient.symmetries().is_rotational() {
                    break;
                }
                quad += Transform::KR;
                tile += Transform::KR;
            }
            if tile.orient.symmetries().is_reflective() {
                break;
            }
            quad += Transform::FU;
            tile += Transform::FU;
        }
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
