#[cfg(test)]
mod tests;

use super::{
    orientation::{Orient, Symmetries, Transform},
    path::TilePos,
    presets::{tiles::*, QUADS},
    tile::{Quad, SubTile, Tile},
};
use std::collections::HashMap;

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileFill {
    Empty,
    Partial,
    Full,
    Leaf,
}

impl TileFill {
    pub fn is_leaf(self) -> bool {
        matches!(self, Self::Empty | Self::Leaf)
    }

    pub fn is_full(self) -> bool {
        matches!(self, Self::Full | Self::Leaf)
    }

    fn infer(quad: Quad<Self>) -> Self {
        use TileFill::*;
        let mut info = None;
        for child in quad.0.into_iter() {
            match (info, child) {
                (Some(Partial), _) => unreachable!(),
                (Some(Leaf), _) => unreachable!(),

                (None, Empty) => info = Some(Empty),
                (None, Full | Leaf) => info = Some(Full),

                (Some(Empty), Empty) => {}
                (Some(Full | Leaf), Full | Leaf) => {}

                (_, Partial) => return Partial,
                (Some(Empty), Full | Leaf) => return Partial,
                (Some(Full | Leaf), Empty) => return Partial,
            }
        }
        info.unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlotInfo {
    pub quad: Quad<Tile>,
    pub fill: TileFill,
    pub symmetries: Symmetries,
}

impl SlotInfo {
    pub const SPACE: Self = Self {
        quad: Quad::SPACE,
        fill: TileFill::Empty,
        symmetries: Symmetries::Isotropic,
    };
}

// TODO: these errors happen in Planet parsing, not Fractal.
#[derive(Error, Debug)]
pub enum FractalError {
    #[error(
        "The Zero tile in this planet wasn't space.\n\
        This is probably a bug. Please report the file that causes this."
    )]
    ZeroNotSpace,

    #[error("Tile {0} contained a hole.")]
    NonLeaf(usize),

    #[error("Tile {0} tried to refer to tile {1}, which does not exist.")]
    NonExistentTile(usize, usize),

    #[error("Tile {0} was not upright. It was {1}.")]
    NonCanonOrient(usize, Orient),

    #[error("Tile {0} is a duplicate of Tile {1}. They must all be unique.")]
    Duplicate(usize, usize),

    #[error(
        "Tile {id}'s {sub} tile has id of {sub_id} and orientation of {orient},\n\
        but that tile's symmetries were {symmetries}."
    )]
    WrongSymmetries {
        id: usize,
        sub: SubTile,
        sub_id: usize,
        orient: Orient,
        symmetries: Symmetries,
    },
}
type Result<T> = std::result::Result<T, FractalError>;

// TODO: double check every pub
// separate { recognizer } from Fractal into Biome?
// make Fractal just a normal quadtree with leaf and branch nodes?
// garbage collection

/// a quadtree specialized to not have root nodes,
/// instead relying on reference cycles to create a fractal
#[derive(Debug, Default)]
pub struct Fractal {
    /// the root node of the fractal; the biggest piece
    pub root: Tile,

    /// a mapping from tile id to quadtile
    /// the opposite of recognizer
    pub library: Vec<SlotInfo>,

    /// a mapping from quadtile to tile
    /// the opposite of library
    pub recognizer: HashMap<Quad<Tile>, Tile>,
}

impl Fractal {
    /// creates a default fractal initialized to empty space.
    pub fn new_space() -> Self {
        Self::new(&[Quad::SPACE]).unwrap()
    }

    /// creates a fractal with some leaf tiles.
    /// fails if any of the quads aren't upright or aren't completely filled.
    pub fn new(quads: &[Quad<Tile>]) -> Result<Self> {
        let mut out = Self::default();
        for quad in quads {
            out.register_new(*quad, true, true)?;
        }
        out.finalize_library()
    }

    /// Does validate:
    /// - that all tiles refer to each other with the correct orientations
    ///
    /// Does NOT validate:
    /// - that no leaves other than space contain holes
    /// - that 0 is space
    /// - that space is isotropic and 0 0 0 0
    fn finalize_library(self) -> Result<Self> {
        // all tiles must exist and have the correct symmetries
        for (i, info) in self.library.iter().enumerate() {
            for (st, sub) in info.quad.0.iter().enumerate() {
                let real_sub = self
                    .library
                    .get(sub.id)
                    .ok_or(FractalError::NonExistentTile(i, sub.id))?;

                if sub.orient.symmetries() != real_sub.symmetries {
                    return Err(FractalError::WrongSymmetries {
                        id: i,
                        sub: SubTile::ORDER[st],
                        sub_id: sub.id,
                        orient: sub.orient,
                        symmetries: real_sub.symmetries,
                    });
                }
            }
        }

        Ok(self)
    }

    pub fn get_info(&self, tile_id: usize) -> SlotInfo {
        self.library[tile_id]
    }

    pub fn get(&self, path: TilePos) -> Tile {
        let mut tile = self.root;
        for subtile in path {
            let mut quad = self.library[tile.id].quad;
            quad += tile.orient.to_transform();
            tile = quad[subtile];
        }
        tile
    }

    pub fn set(&mut self, path: TilePos, tile: Tile) -> Tile {
        // expand each child in the path
        let mut cur_tile = self.root;
        let expansions = path
            .into_iter()
            .map(|subtile| {
                let mut quad = self.library[cur_tile.id].quad;
                quad += cur_tile.orient.to_transform();
                cur_tile = quad[subtile];
                (quad, subtile)
            })
            .collect::<Vec<(Quad<Tile>, SubTile)>>();

        if cur_tile == tile {
            return tile;
        }

        // backtrack each expansion, replacing the old nodes with new ones each time
        self.root = expansions
            .into_iter()
            .rev()
            .fold(tile, |cur_node, (mut quad, subtile)| {
                quad[subtile] = cur_node;
                self.register(quad)
            });

        cur_tile
    }

    /// finds (or registers) a quadtile, and returns the Tile { id, orientation }
    fn register(&mut self, quad: Quad<Tile>) -> Tile {
        self.recognizer
            .get(&quad)
            .copied()
            .unwrap_or_else(|| self.register_new(quad, false, true).unwrap())
    }

    /// registers a new quadtile into the library.
    fn register_new(
        &mut self,
        mut quad: Quad<Tile>,
        is_base_tile: bool,
        cache: bool,
    ) -> Result<Tile> {
        let id = self.library.len();
        let orient = quad.reorient();

        let fill = if is_base_tile {
            let is_space = id == 0;
            if !orient.is_upright() {
                return Err(FractalError::NonCanonOrient(id, orient));
            }
            for sub in quad.0 {
                if is_space != (sub.id == 0) {
                    return Err(FractalError::NonLeaf(id));
                }
            }
            if is_space {
                TileFill::Empty
            } else {
                TileFill::Leaf
            }
        } else {
            let sub_info = quad.map(|child| self.library[child.id].fill);
            TileFill::infer(sub_info)
        };
        self.library.push(SlotInfo {
            quad,
            fill,
            symmetries: orient.symmetries(),
        });

        if cache {
            self.cache(
                quad,
                Tile {
                    id,
                    orient: orient.upright(),
                },
            )?;
        }

        Ok(Tile { id, orient })
    }

    // TODO: 6 hash inserts per new tile is probably expensive for the common case.
    fn cache(&mut self, mut quad: Quad<Tile>, mut tile: Tile) -> Result<()> {
        for _reflection in 0..2 {
            for _rotation in 0..3 {
                if let Some(existing) = self.recognizer.insert(quad, tile) {
                    return Err(FractalError::Duplicate(tile.id, existing.id));
                };

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
        Ok(())
    }
}

pub use only_for_reference::Fractal as BoringFractal;
mod only_for_reference {
    use indexmap::IndexSet;
    use std::ops::Index;

    type Quad = [usize; 4];
    type TileId = usize;
    type SubTile = usize;

    #[derive(Debug)]
    pub struct Fractal {
        root: usize,
        nodes: IndexSet<Quad>,
    }

    impl Fractal {
        pub fn new() -> Self {
            let mut nodes = IndexSet::new();
            nodes.insert([0; 4]);
            Self { root: 0, nodes }
        }

        pub fn load(root: TileId, node_array: impl IntoIterator<Item = Quad>) -> Self {
            let nodes = node_array.into_iter().collect::<IndexSet<Quad>>();
            Self { root, nodes }
        }

        pub fn set(&mut self, path: impl IntoIterator<Item = SubTile>, node: TileId) -> TileId {
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
}
