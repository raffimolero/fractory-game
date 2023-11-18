// TODO: fractory game logic

use super::{
    actions::{TargetedAction, TileAction},
    fractal::Fractal,
    orientation::Transform,
    path::{TileOffset, TilePos},
    tile::Tile,
    tree::collision::RawMoveList,
};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    io,
    rc::Rc,
};

use glam::IVec2;

type Behavior = Vec<TargetedAction<TileOffset>>;

#[derive(Debug, Default)]
pub struct BiomeCache {
    biomes: HashMap<BiomeId, Biome>,
}

impl BiomeCache {
    /// inserts a new biomeid-biome pair into the cache.
    pub fn register(&mut self, id: BiomeId, biome: Biome) {
        self.biomes.insert(id, biome);
    }

    pub fn load(&mut self, biome: BiomeId) -> std::io::Result<&Biome> {
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

impl BiomeId {
    /// TODO: FOR TESTING PURPOSES
    pub fn new_xyyy() -> Self {
        Self(Rc::from("xyyy"))
    }
}

/// A single biome, containing information about the fragments within it.
#[derive(Debug)]
pub struct Biome {
    /// How many predefined leaf nodes exist in this biome
    pub leaf_count: usize,

    // Fragment data: struct of arrays
    pub names: Vec<String>,
    behaviors: Vec<Behavior>,
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
        let leaf_count = xyyy.len();
        let mut names = Vec::with_capacity(leaf_count);
        let mut behaviors = Vec::with_capacity(leaf_count);
        for (name, behavior) in xyyy {
            names.push(name.to_string());
            behaviors.push(behavior);
        }
        Self {
            leaf_count,
            names,
            behaviors,
        }
    }
}

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

#[derive(Debug)]
pub struct ActiveTiles(HashSet<TilePos>);

impl ActiveTiles {
    fn new() -> Self {
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

#[derive(Debug)]
pub struct Fractory {
    pub biome: BiomeId,
    pub fractal: Fractal,

    /// Which tiles are activated this tick.
    pub activated: ActiveTiles,

    /// The player's inventory.
    /// Each index corresponds to how many of a tile the player has.
    pub inventory: BTreeMap<usize, usize>,
    // creation_date: Instant,
    // age: Duration,
    // name: String,
}

impl Fractory {
    pub fn load(f: &str) -> io::Result<Self> {
        todo!()
    }

    /// TODO: FOR TESTING PURPOSES
    pub fn new_xyyy(biomes: &mut BiomeCache) -> Self {
        let biome = Biome::new_xyyy();
        let id = BiomeId::new_xyyy();
        biomes.register(id.clone(), biome);
        let mut out = Self {
            biome: id,
            fractal: Fractal::new_xyyy(),
            activated: ActiveTiles::new(),
            inventory: BTreeMap::new(),
        };

        out.fractal.set(TilePos::UNIT, Tile::SPACE);

        enum Config {
            TestZ,
            TestW,
            TestRotor,
            TestGrowFarm,
        }
        let config = Config::TestGrowFarm;

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
            Config::TestRotor => {
                out.fractal.set(
                    TilePos {
                        depth: 3,
                        pos: IVec2 { x: 1, y: 2 },
                        flop: false,
                    },
                    Tile::ROTOR,
                );

                out.fractal.set(
                    TilePos {
                        depth: 3,
                        pos: IVec2 { x: 1, y: 2 },
                        flop: true,
                    },
                    Tile {
                        id: Tile::W.id,
                        orient: Tile::W.orient.rot_cw(),
                    },
                );
                out.activate(TilePos {
                    depth: 3,
                    pos: IVec2 { x: 1, y: 2 },
                    flop: true,
                });
            }
            Config::TestGrowFarm => {
                out.fractal.set(
                    TilePos {
                        depth: 3,
                        pos: IVec2 { x: 0, y: 1 },
                        flop: false,
                    },
                    Tile::W,
                );
                out.fractal.set(
                    TilePos {
                        depth: 3,
                        pos: IVec2 { x: 0, y: 1 },
                        flop: true,
                    },
                    Tile {
                        id: Tile::WIRE.id,
                        orient: Tile::WIRE.orient.rot_cw().rot_cw(),
                    },
                );
                out.activate(TilePos {
                    depth: 3,
                    pos: IVec2 { x: 0, y: 1 },
                    flop: true,
                });
                out.fractal.set(
                    TilePos {
                        depth: 3,
                        pos: IVec2 { x: 1, y: 2 },
                        flop: false,
                    },
                    Tile::GROWER,
                );
                out.fractal.set(
                    TilePos {
                        depth: 3,
                        pos: IVec2 { x: 1, y: 2 },
                        flop: true,
                    },
                    Tile::X,
                );

                out.fractal.set(
                    TilePos {
                        depth: 4,
                        pos: IVec2 { x: 2, y: 6 },
                        flop: false,
                    },
                    Tile {
                        id: Tile::SUCKER.id,
                        orient: Tile::SUCKER.orient.rot_cw().rot_cw(),
                    },
                );
                out.fractal.set(
                    TilePos {
                        depth: 4,
                        pos: IVec2 { x: 3, y: 7 },
                        flop: false,
                    },
                    Tile {
                        id: Tile::SUCKER.id,
                        orient: Tile::SUCKER.orient.rot_cw().rot_cw(),
                    },
                );
                out.fractal.set(
                    TilePos {
                        depth: 4,
                        pos: IVec2 { x: 2, y: 7 },
                        flop: false,
                    },
                    Tile::WIRE,
                );
                out.activate(TilePos {
                    depth: 4,
                    pos: IVec2 { x: 2, y: 7 },
                    flop: false,
                });
                out.fractal.set(
                    TilePos {
                        depth: 4,
                        pos: IVec2 { x: 1, y: 6 },
                        flop: true,
                    },
                    Tile {
                        id: Tile::W.id,
                        orient: Tile::W.orient.rot_cw(),
                    },
                );
                out.fractal.set(
                    TilePos {
                        depth: 4,
                        pos: IVec2 { x: 2, y: 6 },
                        flop: true,
                    },
                    Tile {
                        id: Tile::WIRE.id,
                        orient: Tile::WIRE.orient.rot_cw().rot_cw(),
                    },
                );

                out.fractal.set(
                    TilePos {
                        depth: 4,
                        pos: IVec2 { x: 4, y: 6 },
                        flop: false,
                    },
                    Tile {
                        id: Tile::SUCKER.id,
                        orient: Tile::SUCKER.orient.rot_cw(),
                    },
                );
                out.fractal.set(
                    TilePos {
                        depth: 4,
                        pos: IVec2 { x: 4, y: 7 },
                        flop: false,
                    },
                    Tile {
                        id: Tile::W.id,
                        orient: Tile::W.orient.rot_cw().rot_cw(),
                    },
                );
                out.activate(TilePos {
                    depth: 4,
                    pos: IVec2 { x: 4, y: 7 },
                    flop: false,
                });
                out.fractal.set(
                    TilePos {
                        depth: 4,
                        pos: IVec2 { x: 4, y: 6 },
                        flop: true,
                    },
                    Tile {
                        id: Tile::WIRE.id,
                        orient: Tile::WIRE.orient.rot_cw(),
                    },
                );
            }
        }

        out
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
    pub fn tick(&mut self, biomes: &BiomeCache) {
        // TODO: move poc-fractal/src/tree.rs and poc-fractal/src/tree/collision.rs
        // to be under common/src/sim/logic/actions.rs
        // and finish RawMoveList::apply();

        let Self {
            biome,
            fractal,
            activated: ActiveTiles(activated),
            inventory,
        } = self;

        let Some(biome) = biomes.get(&biome) else {
            // panic in debug mode
            debug_assert!(false, "biome {biome:?} was not loaded before tick.");
            return;
        };

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
                    TileAction::Activate => drop(self.activated.activate(target)),
                }
            }
        }
        let _actions = actions.apply(&mut self.fractal);
    }
}
