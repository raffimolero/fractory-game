use std::{collections::HashMap, ops::Index, sync::Arc};

use glam::IVec2;

use super::{
    presets::tiles::*,
    tile::{Quad, Tile},
};
use crate::sim::logic::{
    actions::{TargetedAction, TileAction},
    orientation::Transform,
    path::TileOffset,
};

pub type Behavior = Vec<TargetedAction<TileOffset>>;

// TODO: bitvec
#[derive(Debug, Clone)]
pub struct Filter(Vec<bool>);

impl Filter {
    pub fn all(frag_count: usize) -> Self {
        Self(vec![true; frag_count])
    }

    pub fn allows(&self, idx: usize) -> bool {
        self.0.get(idx).copied().unwrap_or(false)
    }

    pub fn without(mut self, idx: usize) -> Self {
        self.0[idx] = false;
        self
    }
}

#[derive(Debug, Clone)]
pub struct FragmentData {
    pub quads: Vec<Quad<Tile>>,
    pub leaf_count: usize,
    pub names: Vec<String>,
    pub behaviors: Vec<Behavior>,
}

impl FragmentData {
    pub fn names(&self) -> &[String] {
        &self.names
    }

    pub fn behaviors(&self) -> &[Behavior] {
        &self.behaviors
    }
}

#[derive(Debug)]
pub struct Planet {
    // icon: Icon,
    pub name: String,
    pub desc: String,
    pub fragments: FragmentData,
}

impl Planet {
    pub fn fragments(&self) -> &FragmentData {
        &self.fragments
    }

    pub fn default_id(&self) -> PlanetId {
        PlanetId(self.name.clone().into())
    }
}

#[derive(Debug, Clone)]
pub struct Biome {
    // icon: Icon,
    pub name: String,
    pub desc: String,
    pub fragment_filter: Filter,
    pub starting_tile: Tile,
}

impl Biome {
    pub fn leaf_count(&self) -> usize {
        self.fragment_filter.0.len()
    }

    pub fn default_id(&self) -> BiomeId {
        BiomeId(self.name.clone().into())
    }

    pub fn fragment_filter(&self) -> &Filter {
        &self.fragment_filter
    }

    pub fn behavior<'a>(&self, behaviors: &'a [Behavior], id: usize) -> &'a Behavior {
        const EMPTY: &Behavior = &vec![];
        self.fragment_filter
            .allows(id)
            .then_some(())
            .and_then(|()| behaviors.get(id))
            .unwrap_or(EMPTY)
    }
}

#[derive(Debug)]
pub struct Fragment {
    // icon: Icon,
    name: String,
    desc: String,
    behavior: Behavior,
}

// TODO: move to io
#[derive(Debug, Default)]
pub struct BiomeCache {
    pub biomes: HashMap<BiomeId, Biome>,
}

impl BiomeCache {
    pub fn iter(&self) -> impl Iterator<Item = &BiomeId> {
        self.biomes.keys()
    }

    /// inserts a new biomeid-biome pair into the cache.
    pub fn register(&mut self, id: BiomeId, biome: Biome) {
        self.biomes.insert(id, biome);
    }

    pub fn load(&mut self, id: BiomeId) -> std::io::Result<&Biome> {
        todo!("load a new biome");
    }

    pub fn get_or_load(&mut self, id: BiomeId) -> std::io::Result<&Biome> {
        if self.biomes.contains_key(&id) {
            Ok(&self.biomes[&id])
        } else {
            self.load(id)
        }
    }

    pub fn get(&self, id: &BiomeId) -> Option<&Biome> {
        self.biomes.get(id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BiomeId(pub Arc<str>);

impl<T: Into<Arc<str>>> From<T> for BiomeId {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Default)]
pub struct PlanetCache {
    planets: HashMap<PlanetId, Planet>,
}

impl PlanetCache {
    /// inserts a new biomeid-biome pair into the cache.
    pub fn register(&mut self, id: PlanetId, planet: Planet) {
        self.planets.insert(id, planet);
    }

    pub fn load(&mut self, id: PlanetId) -> std::io::Result<&mut Planet> {
        todo!("load a new biome");
    }

    pub fn get_or_load(&mut self, id: PlanetId) -> std::io::Result<&mut Planet> {
        // polonius pls
        if self.planets.contains_key(&id) {
            Ok(self.planets.get_mut(&id).unwrap())
        } else {
            self.load(id)
        }
    }

    pub fn get(&self, id: &PlanetId) -> Option<&Planet> {
        self.planets.get(id)
    }

    pub fn get_mut(&mut self, id: &PlanetId) -> Option<&mut Planet> {
        self.planets.get_mut(id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlanetId(pub Arc<str>);

impl<T: Into<Arc<str>>> From<T> for PlanetId {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
