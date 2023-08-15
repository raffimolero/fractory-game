// TODO: fractory game logic

use std::collections::{hash_map::Entry, BTreeMap, HashMap};

use super::{
    actions::{TargetedAction, TileAction},
    fractal::Fractal,
    orientation::Transform,
    path::{TileOffset, TilePos},
    tile::Tile,
};

/// A single biome, containing information about the fragments within it.
pub struct Biome {
    // Fragment data: struct of arrays
    names: Vec<String>,
    behaviors: Vec<Vec<TargetedAction<TileOffset>>>,
    // sprites: Vec<Sprite>,

    // /// this will store both leaf and full non-leaf nodes
    // ///
    // /// currently, leaf tiles must be made of 4 full leaf nodes
    // base_library: Vec<(QuadTile, SlotInfo)>,

    // missions: Vec<Mission>,
}

impl Biome {
    /// TODO: FOR TESTING PURPOSES
    pub fn new_xyyy() -> Self {
        Self {
            names: vec!["Space".into(), "X".into(), "Y".into()],
            behaviors: vec![
                // Space: nothing
                vec![],
                // X: store self
                vec![TargetedAction {
                    target: TileOffset {
                        depth: 0,
                        offset: glam::IVec2 { x: 0, y: 0 },
                        flop: false,
                    },
                    act: TileAction::Store,
                }],
                // Y: move East to NE
                vec![TargetedAction {
                    target: TileOffset {
                        depth: 0,
                        offset: glam::IVec2 { x: 1, y: 0 },
                        flop: false,
                    },
                    act: TileAction::Move(
                        TileOffset {
                            depth: 0,
                            offset: glam::IVec2 { x: 0, y: 1 },
                            flop: false,
                        },
                        Transform::KU,
                    ),
                }],
            ],
        }
    }
}

pub struct Fractory {
    pub biome: Biome,
    pub fractal: Fractal,

    /// Which tiles are activated this tick.
    pub activated: HashMap<TilePos, Tile>,

    /// The player's inventory.
    /// Each index corresponds to how many of a tile the player has.
    pub inventory: BTreeMap<usize, usize>,
    // creation_date: Instant,
    // age: Duration,
    // name: String,
}

impl Fractory {
    /// TODO: FOR TESTING PURPOSES
    pub fn new_xyyy() -> Self {
        Self {
            biome: Biome::new_xyyy(),
            fractal: Fractal::new_xyyy(),
            activated: HashMap::new(),
            inventory: BTreeMap::new(),
        }
    }

    pub fn toggle_activation(&mut self, pos: TilePos) {
        match self.activated.entry(pos) {
            Entry::Occupied(entry) => {
                entry.remove_entry();
            }
            Entry::Vacant(entry) => {
                entry.insert(self.fractal.get(pos));
            }
        }
    }

    fn _activate(activated: &mut HashMap<TilePos, Tile>, fractal: &mut Fractal, pos: TilePos) {
        activated.entry(pos).or_insert_with(|| fractal.get(pos));
    }

    pub fn activate(&mut self, pos: TilePos) {
        Self::_activate(&mut self.activated, &mut self.fractal, pos)
    }

    pub fn deactivate(&mut self, pos: TilePos) {
        self.activated.remove(&pos);
    }

    fn _store(fractal: &mut Fractal, inventory: &mut BTreeMap<usize, usize>, pos: TilePos) {
        let tile = fractal.set(pos, Tile::SPACE);
        *inventory.entry(tile.id).or_insert(0) += 1;
    }

    pub fn store(&mut self, pos: TilePos) {
        Self::_store(&mut self.fractal, &mut self.inventory, pos)
    }

    /// Simulates 1 tick of the Fractory.
    pub fn tick(&mut self) {
        // TODO: move poc-fractal/src/tree.rs and poc-fractal/src/tree/collision.rs
        // to be under common/src/sim/logic/actions.rs
        // and finish RawMoveList::apply();

        let Self {
            biome,
            fractal,
            activated,
            inventory,
        } = self;

        // let mut actions = RawMoveList::default();

        let prev_activated = std::mem::replace(activated, HashMap::new());
        for (pos, Tile { id, orient }) in prev_activated {
            let Some(behaviors) = biome.behaviors.get(id) else {
                continue;
            };
            for TargetedAction { target, act } in behaviors {
                let Some(target) = pos + *target else {
                    continue;
                };
                match act {
                    TileAction::Move(destination, transform) => {
                        todo!()
                        // let Some(destination) = pos + *destination else {
                        //     continue;
                        // };
                        // actions.push((target, destination));
                    }
                    TileAction::Store => Self::_store(fractal, inventory, target),
                    TileAction::Activate => Self::_activate(activated, fractal, target),
                }
            }
        }
        // let actions = actions.apply(&mut self.fractal);
    }
}
