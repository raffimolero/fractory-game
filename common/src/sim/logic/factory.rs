// TODO: fractory game logic

use super::{
    actions::{TargetedAction, TileAction},
    fractal::Fractal,
    orientation::Transform,
    path::{TileOffset, TilePos},
    tile::{SubTile, Tile},
    tree::collision::RawMoveList,
};
use std::collections::{hash_map::Entry, BTreeMap, HashMap, HashSet};

use glam::IVec2;

/// A single biome, containing information about the fragments within it.
#[derive(Debug)]
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

type Behavior = Vec<TargetedAction<TileOffset>>;

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

impl Biome {
    /// TODO: FOR TESTING PURPOSES
    pub fn new_xyyy() -> Self {
        Self {
            names: vec![
                "Space".into(),
                "X".into(),
                "Y".into(),
                "Z".into(),
                "W".into(),
            ],
            behaviors: vec![
                // Space
                vec![],
                // X
                vec![],
                // Y
                vec![],
                // Z
                flip_self_and_below_self(),
                // W
                hexagon(),
            ],
        }
    }
}

#[derive(Debug)]
pub struct Fractory {
    pub biome: Biome,
    pub fractal: Fractal,

    /// Which tiles are activated this tick.
    pub activated: HashSet<TilePos>,

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
        let mut out = Self {
            biome: Biome::new_xyyy(),
            fractal: Fractal::new_xyyy(),
            activated: HashSet::new(),
            inventory: BTreeMap::new(),
        };

        out.fractal.set(TilePos::UNIT, Tile::SPACE);

        enum Config {
            TestZ,
            TestW,
        }
        let config = Config::TestW;

        match config {
            Config::TestZ => {
                out.fractal.set(
                    TilePos {
                        depth: 1,
                        pos: IVec2 { x: 0, y: 0 },
                        flop: false,
                    },
                    Tile::Z,
                );

                out.fractal.set(
                    TilePos {
                        depth: 1,
                        pos: IVec2 { x: 0, y: 0 },
                        flop: true,
                    },
                    Tile::Z + Transform::KR,
                );
                out.activate(TilePos {
                    depth: 1,
                    pos: IVec2 { x: 0, y: 0 },
                    flop: true,
                });

                out.fractal.set(
                    TilePos {
                        depth: 1,
                        pos: IVec2 { x: 0, y: 1 },
                        flop: false,
                    },
                    Tile::Z,
                );
                out.fractal.set(
                    TilePos {
                        depth: 2,
                        pos: IVec2 { x: 0, y: 3 },
                        flop: false,
                    },
                    Tile::X,
                );

                out.fractal.set(
                    TilePos {
                        depth: 1,
                        pos: IVec2 { x: 1, y: 1 },
                        flop: false,
                    },
                    Tile::Z,
                );
                out.fractal.set(
                    TilePos {
                        depth: 2,
                        pos: IVec2 { x: 2, y: 3 },
                        flop: false,
                    },
                    Tile::X,
                );
            }
            Config::TestW => {
                out.fractal.set(
                    TilePos {
                        depth: 3,
                        pos: IVec2 { x: 3, y: 5 },
                        flop: false,
                    },
                    Tile::W,
                );
                out.fractal.set(
                    TilePos {
                        depth: 3,
                        pos: IVec2 { x: 3, y: 5 },
                        flop: false,
                    },
                    Tile::SPACE,
                );
                out.activate(TilePos {
                    depth: 3,
                    pos: IVec2 { x: 3, y: 5 },
                    flop: false,
                });

                out.fractal.set(
                    TilePos {
                        depth: 3,
                        pos: IVec2 { x: 1, y: 2 },
                        flop: false,
                    },
                    Tile::W,
                );
                out.activate(TilePos {
                    depth: 3,
                    pos: IVec2 { x: 1, y: 2 },
                    flop: false,
                });
            }
        }

        out
    }

    pub fn toggle_activation(&mut self, pos: TilePos) {
        if !self.activated.insert(pos) {
            self.activated.remove(&pos);
        }
    }

    fn _activate(activated: &mut HashSet<TilePos>, _fractal: &mut Fractal, pos: TilePos) {
        activated.insert(pos);
    }

    pub fn activate(&mut self, pos: TilePos) {
        Self::_activate(&mut self.activated, &mut self.fractal, pos)
    }

    pub fn deactivate(&mut self, pos: TilePos) {
        self.activated.remove(&pos);
    }

    fn _store(fractal: &mut Fractal, inventory: &mut BTreeMap<usize, usize>, pos: TilePos) {
        let tile = fractal.set(pos, Tile::SPACE);
        // let the factory pick up empty tiles for a secret achievement
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

        let mut actions = RawMoveList::default();

        let prev_activated = std::mem::take(activated);
        for pos in prev_activated {
            let Tile { id, orient } = fractal.get(pos);

            let tile_tf = Transform::from(orient);
            let Some(behaviors) = biome.behaviors.get(id) else {
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
                    TileAction::Activate => Self::_activate(activated, fractal, target),
                }
            }
        }
        let _actions = actions.apply(&mut self.fractal);
    }
}
