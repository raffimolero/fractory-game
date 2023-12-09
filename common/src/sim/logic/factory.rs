use super::{
    actions::{RawMoveList, TargetedAction, TileAction},
    fractal::Fractal,
    orientation::Transform,
    path::{TileOffset, TilePos},
    planet::{
        Behavior, Biome, BiomeCache, BiomeId, Filter, FragmentData, Planet, PlanetCache, PlanetId,
    },
    tile::Tile,
};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    io,
    rc::Rc,
};

use glam::IVec2;

#[derive(Debug)]
pub struct ActiveTiles(HashSet<TilePos>);

impl ActiveTiles {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn contains(&self, pos: TilePos) -> bool {
        self.0.contains(&pos)
    }

    /// activates a position.
    ///
    /// returns true if the position used to be inactive.
    pub fn activate(&mut self, pos: TilePos) -> bool {
        self.0.insert(pos)
    }

    /// deactivates a position.
    ///
    /// returns true if the position used to be active.
    pub fn deactivate(&mut self, pos: TilePos) -> bool {
        self.0.remove(&pos)
    }

    /// toggles whether a position is active or inactive.
    ///
    /// returns true if the position is now active.
    pub fn toggle(&mut self, pos: TilePos) -> bool {
        let is_new = self.0.insert(pos);
        if !is_new {
            self.0.remove(&pos);
        }
        is_new
    }
}

/// practically a save file
#[derive(Debug)]
pub struct FractoryMeta {
    pub fractory: Fractory,
    pub planet: PlanetId,
    pub biome: BiomeId,
    // pub creation_date: Instant,
    // pub age: Duration,
    // pub name: String,
}

impl FractoryMeta {
    pub fn new(planet: PlanetId, biome: BiomeId, planet_data: &Planet, biome_data: &Biome) -> Self {
        Self {
            fractory: Fractory::new(planet_data, biome_data),
            planet,
            biome,
        }
    }

    pub fn load(f: &str) -> io::Result<Self> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Fractory {
    pub fractal: Fractal,

    /// Which tiles are activated this tick.
    pub activated: ActiveTiles,

    /// The player's inventory.
    /// Each index corresponds to how many of a tile the player has.
    pub inventory: BTreeMap<usize, usize>,
}

impl Fractory {
    pub fn new(planet: &Planet, biome: &Biome) -> Self {
        let FragmentData {
            quads, leaf_count, ..
        } = &planet.fragments;
        Self {
            fractal: Fractal::new(dbg!(quads), *leaf_count).unwrap(),
            activated: ActiveTiles::new(),
            inventory: BTreeMap::new(),
        }
    }

    pub fn toggle_activation(&mut self, pos: TilePos) {
        self.activated.toggle(pos);
    }
    pub fn activate(&mut self, pos: TilePos) {
        self.activated.activate(pos);
    }
    pub fn deactivate(&mut self, pos: TilePos) {
        self.activated.deactivate(pos);
    }

    fn _store(fractal: &mut Fractal, inventory: &mut BTreeMap<usize, usize>, pos: TilePos) {
        let tile = fractal.get(pos);
        if !fractal.get_info(tile.id).fill.is_full() {
            return;
        }
        fractal.set(pos, Tile::SPACE);
        // let the factory pick up empty tiles for a secret achievement
        *inventory.entry(tile.id).or_insert(0) += 1;
    }

    pub fn store(&mut self, pos: TilePos) {
        Self::_store(&mut self.fractal, &mut self.inventory, pos)
    }

    /// Simulates 1 tick of the Fractory.
    pub fn tick(&mut self, behaviors: &[Behavior], filter: &Filter) {
        let Self {
            fractal,
            activated: ActiveTiles(activated),
            inventory,
        } = self;

        let mut actions = RawMoveList::default();

        let prev_activated = std::mem::take(activated);
        for pos in prev_activated {
            let Tile { id, orient } = fractal.get(pos);

            let tile_tf = orient.to_transform();
            let Some(behaviors) = filter
                .allows(id)
                .then_some(())
                .and_then(|_| behaviors.get(id))
            else {
                continue;
            };

            for TargetedAction { mut target, act } in behaviors.iter().copied() {
                target += tile_tf;
                let Some(target) = pos + target else {
                    continue;
                };
                match act {
                    TileAction::Move(mut destination, transform) => {
                        destination += tile_tf;
                        let Some(destination) = pos + destination else {
                            continue;
                        };
                        actions.add(target, destination, tile_tf * transform);
                    }
                    TileAction::Store => Self::_store(fractal, inventory, target),
                    TileAction::Activate => drop(self.activated.activate(target)),
                }
            }
        }
        let _actions = actions.apply(&mut self.fractal);
    }
}
