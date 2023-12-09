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
    Full { is_leaf: bool },
}

impl TileFill {
    pub fn is_leaf(self) -> bool {
        matches!(self, Self::Empty | Self::Full { is_leaf: true })
    }

    pub fn is_full(self) -> bool {
        matches!(self, Self::Full { .. })
    }

    fn infer(quad: Quad<Self>) -> Self {
        use TileFill::*;
        let mut info = None;
        for child in quad.0.into_iter() {
            match (info, child) {
                (Some(Partial), _) => unreachable!(),
                (Some(Full { is_leaf: true }), _) => unreachable!(),

                (None, Empty) => info = Some(Empty),
                (None, Full { .. }) => info = Some(Full { is_leaf: false }),

                (Some(Empty), Empty) => {}
                (Some(Full { .. }), Full { .. }) => {}

                (_, Partial) => return Partial,
                (Some(Empty), Full { .. }) => return Partial,
                (Some(Full { .. }), Empty) => return Partial,
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

#[derive(Error, Debug)]
pub enum FractalError {
    #[error("The Zero tile in this planet wasn't space.")]
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
        "Tile {0}'s {1} tile has id of {2} and orientation of {3},\n\
        but that tile's symmetries were {4}."
    )]
    WrongSymmetries(usize, SubTile, usize, Orient, Symmetries),
}
type Result<T> = std::result::Result<T, FractalError>;

// TODO: double check every pub
// separate { recognizer, leaf_count } from Fractal into Biome
// make Fractal just a normal quadtree with leaf and branch nodes
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
        Self::new(&[Quad::SPACE], 0).unwrap()
    }

    /// creates a fractal with some leaf tiles.
    /// fails if any of the quads aren't upright or aren't completely filled.
    pub fn new(quads: &[Quad<Tile>], leaf_count: usize) -> Result<Self> {
        let mut out = Self::default();

        if quads[0] != Quad::SPACE {
            return Err(FractalError::ZeroNotSpace);
        }
        out.register_leaf(Quad::SPACE).unwrap();
        out.library[0].fill = TileFill::Empty;

        let (leaves, intermediates) = quads[1..].split_at(leaf_count);
        for quad in leaves.iter().copied() {
            out.register_leaf(quad)?;
        }
        for quad in intermediates.iter().copied() {
            out.register_new(quad)?;
        }
        out.validate_library(leaf_count)?;
        Ok(out)
    }

    /// Does validate:
    /// - that all tiles refer to each other with the correct orientations
    ///
    /// Does NOT validate:
    /// - that 0 is space
    /// - that space is isotropic and 0 0 0 0
    fn validate_library(&self, leaf_count: usize) -> Result<()> {
        for (i, info) in self.library.iter().enumerate() {
            if i <= leaf_count {
                let subtile_fills = info.quad.map(|child| self.library[child.id].fill);
                let fill = TileFill::infer(subtile_fills);
                if !fill.is_full() {
                    return Err(FractalError::NonLeaf(i));
                }
                assert!(info.fill.is_leaf());
            }
            for (st, sub) in info.quad.0.iter().enumerate() {
                let real_sub = self
                    .library
                    .get(sub.id)
                    .ok_or(FractalError::NonExistentTile(i, sub.id))?;
                if sub.orient.symmetries() != real_sub.symmetries {
                    return Err(FractalError::WrongSymmetries(
                        i,
                        SubTile::ORDER[st],
                        sub.id,
                        sub.orient,
                        real_sub.symmetries,
                    ));
                }
            }
        }

        Ok(())
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
            .unwrap_or_else(|| self.register_new(quad).unwrap())
    }

    /// registers a new leaf quadtile into the library.
    /// returns Err if the tile isn't upright, or if it has a hole.
    fn register_leaf(&mut self, mut quad: Quad<Tile>) -> Result<()> {
        let id = self.library.len();
        let orient = quad.reorient();
        if !orient.is_upright() {
            return Err(FractalError::NonCanonOrient(id, orient));
        }

        self.library.push(SlotInfo {
            quad,
            fill: TileFill::Full { is_leaf: true },
            symmetries: orient.symmetries(),
        });

        self.cache(quad, Tile { id, orient })
    }

    /// registers a new non-leaf quadtile into the library.
    fn register_new(&mut self, mut quad: Quad<Tile>) -> Result<Tile> {
        let orient = quad.reorient();
        let id = self.library.len();

        let sub_info = quad.map(|child| self.library[child.id].fill);
        self.library.push(SlotInfo {
            quad,
            fill: TileFill::infer(sub_info),
            symmetries: orient.symmetries(),
        });

        self.cache(
            quad,
            Tile {
                id,
                orient: orient.upright(),
            },
        )?;

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
