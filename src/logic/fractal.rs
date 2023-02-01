use crate::logic::path::SubTile;

use super::orientation::{Orient, Transform};
use std::{
    collections::HashMap,
    ops::{Add, AddAssign, Index, IndexMut},
};

use indexmap::IndexSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Tile {
    id: usize,
    orient: Orient,
}

impl AddAssign<Transform> for Tile {
    fn add_assign(&mut self, rhs: Transform) {
        self.orient += rhs;
    }
}

impl Add<Transform> for Tile {
    type Output = Self;

    fn add(mut self, rhs: Transform) -> Self::Output {
        self += rhs;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Tringle<T>([T; 4]);

impl<T> Index<SubTile> for Tringle<T> {
    type Output = T;

    fn index(&self, index: SubTile) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl<T> IndexMut<SubTile> for Tringle<T> {
    fn index_mut(&mut self, index: SubTile) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

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

impl Tringle<Tile> {
    fn is_upright_and_reflective(self) -> bool {
        use Orient::*;
        use SubTile::*;
        matches!(self[C].orient, Iso | RfU) // core is reflective and upright
            && matches!(self[U].orient, Iso | RfU) // upper is reflective and upright
            && self[R] + Transform::FR == self[L] // wings match
    }

    fn is_rotational(self) -> bool {
        use SubTile::*;
        let uu = self[U];
        let ru = self[R] + Transform::KL;
        let lu = self[L] + Transform::KR;
        self[C].orient.symmetries().is_rotational() && uu == ru && uu == lu
    }

    /// reorients a tringle upright, and returns its original orientation.
    fn reorient(&mut self) -> Orient {
        use Orient::*;
        let is_rot = self.is_rotational();

        let mut is_ref = false;
        for i in 0..3 {
            if self.is_upright_and_reflective() {
                return if is_rot { Iso } else { [RfU, RfL, RfR][i] };
            }
            if is_rot {
                return RtK;
            }
            *self += Transform::KR;
        }
        AKU
    }
}

struct Fractory {
    recognizer: HashMap<Tringle<Tile>, Tile>,
    library: Vec<Tringle<Tile>>,
    root: Tile,
    // TODO: behavior: Vec<Behavior>,
}

impl Fractory {
    fn identify(&self, tringle: &Tringle<Tile>) -> Option<&Tile> {
        self.recognizer.get(tringle)
    }

    fn register(&mut self, tringle: Tringle<Tile>) -> Tile {
        if let Some(tile) = self.identify(&tringle) {
            *tile
        } else {
            self.register_unchecked(tringle)
        }
    }

    fn cache_symmetries(&mut self, mut tringle: Tringle<Tile>, mut tile: Tile) {
        for _reflection in 0..2 {
            for _rotation in 0..3 {
                let result = self.recognizer.insert(tringle, tile);
                assert!(
                    result.is_none(),
                    "Tringle ({tringle:?}) was registered ({tile:?}) twice!"
                );

                if tile.orient.symmetries().is_rotational() {
                    break;
                }
                tringle += Transform::KR;
                tile += Transform::KR;
            }
            if tile.orient.symmetries().is_reflective() {
                break;
            }
            tringle += Transform::FU;
            tile += Transform::FU;
        }
    }

    fn register_unchecked(&mut self, mut tringle: Tringle<Tile>) -> Tile {
        let orient = tringle.reorient();
        let id = self.library.len();
        self.library.push(tringle);

        self.cache_symmetries(
            tringle,
            Tile {
                id,
                orient: orient.upright(),
            },
        );

        Tile { id, orient }
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
