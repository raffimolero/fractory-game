use std::{collections::HashMap, ops::Index, rc::Rc};

use glam::IVec2;

use crate::sim::logic::{
    actions::{TargetedAction, TileAction},
    orientation::Transform,
    path::TileOffset,
};

use super::tile::Tile;

pub type Behavior = Vec<TargetedAction<TileOffset>>;

// TODO: bitvec
#[derive(Debug, Clone)]
pub struct Filter(Vec<bool>);

impl Filter {
    fn all(frag_count: usize) -> Self {
        Self(vec![true; frag_count])
    }

    pub fn allows(&self, idx: usize) -> bool {
        self.0.get(idx).copied().unwrap_or(false)
    }

    fn without(mut self, idx: usize) -> Self {
        self.0[idx] = false;
        self
    }
}

#[derive(Debug, Clone)]
pub struct FragmentData {
    names: Vec<String>,
    behaviors: Vec<Behavior>,
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
    name: String,
    desc: String,
    fragments: FragmentData,
    biomes: BiomeCache,
}

impl Planet {
    /// TODO: FOR TESTING PURPOSES
    pub fn new_xyyy() -> Self {
        let xyyy = [
            ("", vec![]),
            ("X", vec![]),
            ("Y", vec![]),
            ("Flip-Flop", flip_self_and_below_self()),
            ("Spinner", hexagon()),
            ("Rotor", rotate()),
            ("Grower", grow()),
            ("Sucker", suck()),
            ("Wire", wire()),
        ];
        let frag_count = xyyy.len();
        let mut names = Vec::with_capacity(frag_count);
        let mut behaviors = Vec::with_capacity(frag_count);
        for (name, behavior) in xyyy {
            names.push(name.to_string());
            behaviors.push(behavior);
        }

        Self {
            name: "XYYY".into(),
            desc: "The first planet.".into(),
            fragments: FragmentData { names, behaviors },
            biomes: BiomeCache::new_xyyy(frag_count),
        }
    }

    pub fn fragments(&self) -> &FragmentData {
        &self.fragments
    }

    pub fn biomes(&self) -> &BiomeCache {
        &self.biomes
    }

    pub fn biomes_mut(&mut self) -> &mut BiomeCache {
        &mut self.biomes
    }

    pub fn default_id(&self) -> PlanetId {
        PlanetId(self.name.clone().into())
    }
}

#[derive(Debug, Clone)]
pub struct Biome {
    // icon: Icon,
    name: String,
    desc: String,
    fragment_filter: Filter,
}

impl Biome {
    /// TODO: FOR TESTING PURPOSES
    pub fn new_xyyy_spinless(frag_count: usize) -> Self {
        Self {
            name: "Spinless".into(),
            desc: "Disables rotors and spinners.".into(),
            fragment_filter: Filter::all(frag_count)
                .without(Tile::ROTOR.id)
                .without(Tile::W.id),
        }
    }

    /// TODO: FOR TESTING PURPOSES
    pub fn new_xyyy_landing_zone(frag_count: usize) -> Self {
        Self {
            name: "Landing Zone".into(),
            desc: "Contains every fragment.".into(),
            fragment_filter: Filter::all(frag_count),
        }
    }

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

// TODO: FOR TESTING PURPOSES
fn swap_01_with_10() -> Behavior {
    vec![
        TargetedAction {
            target: TileOffset {
                depth: 0,
                offset: IVec2 { x: 1, y: 0 },
                flop: false,
            },
            act: TileAction::Store,
        },
        TargetedAction {
            target: TileOffset {
                depth: 0,
                offset: IVec2 { x: 0, y: 1 },
                flop: false,
            },
            act: TileAction::Move(
                TileOffset {
                    depth: 0,
                    offset: IVec2 { x: 1, y: 0 },
                    flop: false,
                },
                Transform::KU,
            ),
        },
    ]
}

fn flip_self_and_below_self() -> Behavior {
    let this = TileOffset::ZERO;
    let below = TileOffset {
        depth: 0,
        offset: IVec2::ZERO,
        flop: true,
    };
    vec![
        TargetedAction {
            target: this,
            act: TileAction::Move(this, Transform::FU),
        },
        TargetedAction {
            target: below,
            act: TileAction::Move(below, Transform::FU),
        },
    ]
}

fn hexagon() -> Behavior {
    let this = TileOffset::ZERO;
    let below = TileOffset {
        depth: 0,
        offset: IVec2::ZERO,
        flop: true,
    };
    vec![
        TargetedAction {
            target: this,
            act: TileAction::Move(below, Transform::KR),
        },
        TargetedAction {
            target: below,
            act: TileAction::Activate,
        },
    ]
}

fn rotate() -> Behavior {
    let this = TileOffset::ZERO;
    let u = TileOffset {
        depth: 0,
        offset: IVec2::ZERO,
        flop: true,
    };
    let l = TileOffset {
        depth: 0,
        offset: IVec2::new(0, -1),
        flop: true,
    };
    let r = TileOffset {
        depth: 0,
        offset: IVec2::new(-1, -1),
        flop: true,
    };
    vec![
        TargetedAction {
            target: u,
            act: TileAction::Move(r, Transform::KR),
        },
        TargetedAction {
            target: r,
            act: TileAction::Move(l, Transform::KR),
        },
        TargetedAction {
            target: l,
            act: TileAction::Move(u, Transform::KR),
        },
        TargetedAction {
            target: this,
            act: TileAction::Activate,
        },
    ]
}

fn grow() -> Behavior {
    let below = TileOffset {
        depth: 0,
        offset: IVec2::ZERO,
        flop: true,
    };
    let center_below = TileOffset {
        depth: 1,
        offset: IVec2::new(1, 2),
        flop: false,
    };
    vec![TargetedAction {
        target: center_below,
        act: TileAction::Move(below, Transform::KU),
    }]
}

fn suck() -> Behavior {
    let below = TileOffset {
        depth: 0,
        offset: IVec2::ZERO,
        flop: true,
    };
    vec![TargetedAction {
        target: below,
        act: TileAction::Store,
    }]
}

fn wire() -> Behavior {
    vec![
        TargetedAction {
            target: TileOffset {
                depth: 0,
                offset: IVec2::new(0, -1),
                flop: true,
            },
            act: TileAction::Activate,
        },
        TargetedAction {
            target: TileOffset {
                depth: 0,
                offset: IVec2::new(-1, -1),
                flop: true,
            },
            act: TileAction::Activate,
        },
    ]
}

// TODO: move to io
#[derive(Debug, Default)]
pub struct BiomeCache {
    biomes: HashMap<BiomeId, Biome>,
}

impl BiomeCache {
    /// TODO: FOR TESTING PURPOSES
    fn new_xyyy(frag_count: usize) -> Self {
        Self {
            biomes: HashMap::from(
                [
                    Biome::new_xyyy_spinless(frag_count),
                    Biome::new_xyyy_landing_zone(frag_count),
                ]
                .map(|b| (b.default_id(), b)),
            ),
        }
    }

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
pub struct BiomeId(Rc<str>);

impl<T: Into<Rc<str>>> From<T> for BiomeId {
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
pub struct PlanetId(Rc<str>);
