// TODO: fractory game logic

use super::{
    actions::{TargetedAction, TileAction},
    fractal::Fractal,
    orientation::Transform,
    path::{TileOffset, TilePos},
    planet::{Behavior, Biome, BiomeCache, BiomeId, Filter, Planet, PlanetCache, PlanetId},
    tile::Tile,
    tree::collision::RawMoveList,
};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    io,
    rc::Rc,
};

use glam::IVec2;

// TODO: remove comment
// /// A single planet, containing information about the fragments within it.
// #[derive(Debug)]
// pub struct Biome {
//     /// How many predefined leaf nodes exist in this biome
//     pub leaf_count: usize,

//     // Fragment data: struct of arrays
//     pub names: Vec<String>,
//     behaviors: Vec<Behavior>,
//     // sprites: Vec<Sprite>,

//     // /// this will store both leaf and full non-leaf nodes
//     // ///
//     // /// currently, leaf tiles must be made of 4 full leaf nodes
//     // base_library: Vec<(QuadTile, SlotInfo)>,

//     // only in biomes
//     // missions: Vec<Mission>,
// }

// impl Biome {
//     /// TODO: FOR TESTING PURPOSES
//     pub fn new_xyyy() -> Self {
//         let xyyy = [
//             ("", vec![]),
//             ("X", vec![]),
//             ("Y", vec![]),
//             ("Flip-Flop", flip_self_and_below_self()),
//             ("Spinner", hexagon()),
//             ("Rotor", rotate()),
//             ("Grower", grow()),
//             ("Sucker", suck()),
//             ("Wire", wire()),
//         ];
//         let leaf_count = xyyy.len();
//         let mut names = Vec::with_capacity(leaf_count);
//         let mut behaviors = Vec::with_capacity(leaf_count);
//         for (name, behavior) in xyyy {
//             names.push(name.to_string());
//             behaviors.push(behavior);
//         }
//         Self {
//             leaf_count,
//             names,
//             behaviors,
//         }
//     }
// }

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
    pub fn load(f: &str) -> io::Result<Self> {
        todo!()
    }

    /// TODO: FOR TESTING PURPOSES
    pub fn new_xyyy(planets: &mut PlanetCache) -> Self {
        let xyyy = Planet::new_xyyy();
        let planet_id = xyyy.default_id();
        let biome_id = BiomeId::from("Spinless");
        planets.register(planet_id.clone(), xyyy);
        Self {
            fractory: Fractory::new_xyyy(),
            planet: planet_id,
            biome: biome_id,
        }
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
    /// TODO: FOR TESTING PURPOSES
    pub fn rot_cw(&mut self) {
        let main = self.fractal.get(TilePos::UNIT);

        self.fractal.set(
            TilePos::UNIT,
            Tile {
                id: main.id,
                orient: main.orient.rot_cw(),
            },
        );

        let mut new_set = HashSet::new();
        for pos in &self.activated.0 {
            let mut new_pos = TilePos::UNIT;
            for subtile in *pos {
                new_pos.push_back(subtile + Transform::KR);
            }
            new_set.insert(new_pos);
        }
        self.activated.0 = new_set;
    }

    /// TODO: FOR TESTING PURPOSES
    pub fn new_xyyy() -> Self {
        let mut out = Self {
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
            TestGrowBug,
            TestActiveBug,
        }
        let config = Config::TestActiveBug;

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
            Config::TestGrowBug => {
                out.fractal.set(
                    TilePos {
                        depth: 3,
                        pos: IVec2 { x: 1, y: 2 },
                        flop: false,
                    },
                    Tile {
                        id: Tile::GROWER.id,
                        orient: Tile::GROWER.orient.rot_cw(),
                    },
                );
                out.activate(TilePos {
                    depth: 3,
                    pos: IVec2 { x: 1, y: 2 },
                    flop: false,
                });
                out.fractal.set(
                    TilePos {
                        depth: 4,
                        pos: IVec2 { x: 1, y: 4 },
                        flop: false,
                    },
                    Tile::X,
                );

                out.fractal.set(
                    TilePos {
                        depth: 3,
                        pos: IVec2 { x: 2, y: 2 },
                        flop: false,
                    },
                    Tile::GROWER,
                );
                out.activate(TilePos {
                    depth: 3,
                    pos: IVec2 { x: 2, y: 2 },
                    flop: false,
                });
                out.fractal.set(
                    TilePos {
                        depth: 4,
                        pos: IVec2 { x: 5, y: 6 },
                        flop: false,
                    },
                    Tile::X,
                );
            }
            Config::TestActiveBug => {
                out.fractal.set(
                    TilePos {
                        depth: 4,
                        pos: IVec2 { x: 2, y: 5 },
                        flop: true,
                    },
                    Tile::W,
                );
                out.activate(TilePos {
                    depth: 4,
                    pos: IVec2 { x: 2, y: 5 },
                    flop: true,
                });

                out.fractal.set(
                    TilePos {
                        depth: 4,
                        pos: IVec2 { x: 3, y: 5 },
                        flop: true,
                    },
                    Tile {
                        id: Tile::W.id,
                        orient: Tile::W.orient.flip(),
                    },
                );
                out.activate(TilePos {
                    depth: 4,
                    pos: IVec2 { x: 3, y: 5 },
                    flop: true,
                });
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
    pub fn tick(&mut self, behaviors: &[Behavior], filter: &Filter) {
        // TODO: move poc-fractal/src/tree.rs and poc-fractal/src/tree/collision.rs
        // to be under common/src/sim/logic/actions.rs
        // and finish RawMoveList::apply();

        let Self {
            fractal,
            activated: ActiveTiles(activated),
            inventory,
        } = self;

        // let Some(biome) = biomes.get(&biome) else {
        //     // panic in debug mode
        //     debug_assert!(false, "biome {biome:?} was not loaded before tick.");
        //     return;
        // };

        let mut actions = RawMoveList::default();

        let prev_activated = std::mem::take(activated);
        for pos in prev_activated {
            let Tile { id, orient } = fractal.get(pos);

            let tile_tf = orient.transform();
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
