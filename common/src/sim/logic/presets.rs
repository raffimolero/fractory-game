use std::{
    collections::{BTreeMap, HashMap},
    str::FromStr,
};

use super::{
    actions::{TargetedAction, TileAction},
    factory::{ActiveTiles, Fractory, FractoryMeta},
    fractal::Fractal,
    orientation::Transform,
    orientation::{Orient, Transform::*},
    path::{TileOffset, TilePos},
    planet::{Behavior, Biome, BiomeCache, BiomeId, Filter, FragmentData, Planet, PlanetCache},
    tile::{Quad, Tile},
};

pub mod tiles {
    pub const SPACE: usize = 0;
    pub const X: usize = 1;
    pub const Y: usize = 2;
    pub const FLIP_FLOP: usize = 3;
    pub const SPINNER: usize = 4;
    pub const ROTOR: usize = 5;
    pub const GROWER: usize = 6;
    pub const SUCKER: usize = 7;
    pub const WIRE: usize = 8;

    pub const LEAF_COUNT: usize = 8;
    pub const TILE_COUNT: usize = LEAF_COUNT + 1;
}
use tiles::*;

pub const TILES: [Tile; TILE_COUNT] = {
    use Orient::*;
    const ORIENTS: [Orient; TILE_COUNT] = [
        Iso, // SPACE
        Iso, // X
        Iso, // Y
        RfU, // FLIP_FLOP
        AKU, // SPINNER
        RtK, // ROTOR
        RfU, // GROWER
        RfU, // SUCKER
        RfU, // WIRE
    ];

    let mut out = [Tile::SPACE; TILE_COUNT];
    let mut i = 0;
    while i < LEAF_COUNT {
        out[i] = Tile {
            id: i,
            orient: ORIENTS[i],
        };
        i += 1;
    }
    out
};

pub const QUADS: [Quad<Tile>; TILE_COUNT] = {
    const INDEX_TF: [[(usize, Transform); 4]; TILE_COUNT] = [
        [(0, KU); 4],                         // SPACE
        [(1, KU), (2, KU), (2, KU), (2, KU)], // X
        [(2, KU), (1, KU), (1, KU), (1, KU)], // Y
        [(1, KU), (1, KU), (2, KU), (2, KU)], // FLIP_FLOP
        [(3, KU), (1, KU), (2, KU), (1, KU)], // SPINNER
        [(1, KU), (3, KR), (3, KL), (3, KU)], // ROTOR
        [(3, KU), (1, KU), (2, KU), (2, KU)], // GROWER
        [(3, KU), (2, KU), (1, KU), (1, KU)], // SUCKER
        [(2, KU), (2, KU), (1, KU), (1, KU)], // WIRE
    ];

    let mut out = [Quad::SPACE; TILE_COUNT];
    let mut i = 0;
    while i < TILE_COUNT {
        let mut j = 0;
        while j < 4 {
            let (id, tf) = INDEX_TF[i][j];
            out[i].0[j] = TILES[id].transformed(tf);
            j += 1;
        }
        i += 1;
    }
    out
};

pub const XYYY: &'static str = "xyyy";
pub const XYYY_LANDING_ZONE: &'static str = "Landing Zone";
pub const XYYY_SPINLESS: &'static str = "Spinless";

pub fn new_xyyy_fractal() -> Fractal {
    // TODO: take fragments as argument
    Fractal::new(&QUADS[1..]).unwrap()
}

pub fn new_xyyy_fractory_meta(planets: &mut PlanetCache) -> FractoryMeta {
    let xyyy = new_xyyy_planet();
    let planet_id = xyyy.default_id();
    // let biome_id = BiomeId::from(XYYY_SPINLESS);
    let biome_id = BiomeId::from(XYYY_LANDING_ZONE);
    planets.register(planet_id.clone(), xyyy);
    FractoryMeta {
        fractory: new_xyyy_fractory(),
        planet: planet_id,
        biome: biome_id,
    }
}
pub fn new_xyyy_planet() -> Planet {
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

    Planet {
        name: "XYYY".into(),
        desc: "The first planet.".into(),
        fragments: FragmentData {
            quads: QUADS.to_vec(),
            names,
            behaviors,
        },
    }
}

pub fn new_xyyy_biome_spinless() -> Biome {
    Biome {
        name: XYYY_SPINLESS.into(),
        desc: "Disables rotors and spinners.".into(),
        fragment_filter: Filter::all(LEAF_COUNT).without(ROTOR).without(SPINNER),
        starting_tile: TILES[Y],
    }
}

pub fn new_xyyy_biome_landing_zone() -> Biome {
    Biome {
        name: XYYY_LANDING_ZONE.into(),
        desc: "Contains every fragment.".into(),
        fragment_filter: Filter::all(LEAF_COUNT),
        starting_tile: TILES[X],
    }
}

pub fn new_xyyy_biome_cache() -> BiomeCache {
    BiomeCache {
        biomes: HashMap::from(
            [new_xyyy_biome_spinless(), new_xyyy_biome_landing_zone()].map(|b| (b.default_id(), b)),
        ),
    }
}

fn flip_self_and_below_self() -> Behavior {
    let this = TileOffset::ZERO;
    let below = TileOffset {
        depth: 0,
        offset: (0, 0).into(),
        flop: true,
    };
    vec![
        TargetedAction {
            target: this,
            act: TileAction::Move(this, FU),
        },
        TargetedAction {
            target: below,
            act: TileAction::Move(below, FU),
        },
    ]
}

fn hexagon() -> Behavior {
    let this = TileOffset::ZERO;
    let below = TileOffset {
        depth: 0,
        offset: (0, 0).into(),
        flop: true,
    };
    vec![
        TargetedAction {
            target: this,
            act: TileAction::Move(below, KR),
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
        offset: (0, 0).into(),
        flop: true,
    };
    let l = TileOffset {
        depth: 0,
        offset: (0, -1).into(),
        flop: true,
    };
    let r = TileOffset {
        depth: 0,
        offset: (-1, -1).into(),
        flop: true,
    };
    vec![
        TargetedAction {
            target: u,
            act: TileAction::Move(r, KR),
        },
        TargetedAction {
            target: r,
            act: TileAction::Move(l, KR),
        },
        TargetedAction {
            target: l,
            act: TileAction::Move(u, KR),
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
        offset: (0, 0).into(),
        flop: true,
    };
    let center_below = TileOffset {
        depth: 1,
        offset: (1, 2).into(),
        flop: false,
    };
    vec![TargetedAction {
        target: center_below,
        act: TileAction::Move(below, KU),
    }]
}

fn suck() -> Behavior {
    let below = TileOffset {
        depth: 0,
        offset: (0, 0).into(),
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
                offset: (0, -1).into(),
                flop: true,
            },
            act: TileAction::Activate,
        },
        TargetedAction {
            target: TileOffset {
                depth: 0,
                offset: (-1, -1).into(),
                flop: true,
            },
            act: TileAction::Activate,
        },
    ]
}

struct PlaceCommand {
    depth: u8,
    x: i32,
    y: i32,
    flop: bool,
    tile: Tile,
    active: bool,
}

/// NOT a save file.
struct Preset(Vec<PlaceCommand>);

impl FromStr for Preset {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // for line in s.lines() {}
        Err("todo")
    }
}

pub fn new_xyyy_fractory() -> Fractory {
    let mut out = Fractory {
        fractal: new_xyyy_fractal(),
        activated: ActiveTiles::new(),
        inventory: BTreeMap::new(),
    };

    enum Config {
        Empty,
        TestZ,
        TestW,
        TestRotor,
        TestGrowFarm,
        TestGrowBug,
        TestActiveBug,
    }
    let config = Config::TestGrowFarm;

    match config {
        Config::Empty => {}
        Config::TestZ => {
            out.fractal.set(
                TilePos {
                    depth: 1,
                    pos: (0, 0).into(),
                    flop: false,
                },
                TILES[FLIP_FLOP],
            );

            out.fractal.set(
                TilePos {
                    depth: 1,
                    pos: (0, 0).into(),
                    flop: true,
                },
                TILES[FLIP_FLOP] + KR,
            );
            out.activate(TilePos {
                depth: 1,
                pos: (0, 0).into(),
                flop: true,
            });

            out.fractal.set(
                TilePos {
                    depth: 1,
                    pos: (0, 1).into(),
                    flop: false,
                },
                TILES[FLIP_FLOP],
            );
            out.fractal.set(
                TilePos {
                    depth: 2,
                    pos: (0, 3).into(),
                    flop: false,
                },
                TILES[X],
            );

            out.fractal.set(
                TilePos {
                    depth: 1,
                    pos: (1, 1).into(),
                    flop: false,
                },
                TILES[FLIP_FLOP],
            );
            out.fractal.set(
                TilePos {
                    depth: 2,
                    pos: (2, 3).into(),
                    flop: false,
                },
                TILES[X],
            );
        }
        Config::TestW => {
            out.fractal.set(
                TilePos {
                    depth: 3,
                    pos: (3, 5).into(),
                    flop: false,
                },
                TILES[SPINNER],
            );
            out.fractal.set(
                TilePos {
                    depth: 3,
                    pos: (3, 5).into(),
                    flop: false,
                },
                TILES[SPACE],
            );
            out.activate(TilePos {
                depth: 3,
                pos: (3, 5).into(),
                flop: false,
            });

            out.fractal.set(
                TilePos {
                    depth: 3,
                    pos: (1, 2).into(),
                    flop: false,
                },
                TILES[SPINNER],
            );
            out.activate(TilePos {
                depth: 3,
                pos: (1, 2).into(),
                flop: false,
            });
        }
        Config::TestRotor => {
            out.fractal.set(
                TilePos {
                    depth: 3,
                    pos: (1, 2).into(),
                    flop: false,
                },
                TILES[ROTOR],
            );

            out.fractal.set(
                TilePos {
                    depth: 3,
                    pos: (1, 2).into(),
                    flop: true,
                },
                TILES[SPINNER] + KR,
            );
            out.activate(TilePos {
                depth: 3,
                pos: (1, 2).into(),
                flop: true,
            });
        }
        Config::TestGrowFarm => {
            out.fractal.set(
                TilePos {
                    depth: 3,
                    pos: (0, 1).into(),
                    flop: false,
                },
                TILES[SPINNER],
            );
            out.fractal.set(
                TilePos {
                    depth: 3,
                    pos: (0, 1).into(),
                    flop: true,
                },
                TILES[WIRE] + KL,
            );
            out.activate(TilePos {
                depth: 3,
                pos: (0, 1).into(),
                flop: true,
            });
            out.fractal.set(
                TilePos {
                    depth: 3,
                    pos: (1, 2).into(),
                    flop: false,
                },
                TILES[GROWER],
            );
            out.fractal.set(
                TilePos {
                    depth: 3,
                    pos: (1, 2).into(),
                    flop: true,
                },
                TILES[X],
            );

            out.fractal.set(
                TilePos {
                    depth: 4,
                    pos: (2, 6).into(),
                    flop: false,
                },
                TILES[SUCKER] + KL,
            );
            out.fractal.set(
                TilePos {
                    depth: 4,
                    pos: (3, 7).into(),
                    flop: false,
                },
                TILES[SUCKER] + KL,
            );
            out.fractal.set(
                TilePos {
                    depth: 4,
                    pos: (2, 7).into(),
                    flop: false,
                },
                TILES[WIRE],
            );
            out.activate(TilePos {
                depth: 4,
                pos: (2, 7).into(),
                flop: false,
            });
            out.fractal.set(
                TilePos {
                    depth: 4,
                    pos: (1, 6).into(),
                    flop: true,
                },
                TILES[SPINNER] + KR,
            );
            out.fractal.set(
                TilePos {
                    depth: 4,
                    pos: (2, 6).into(),
                    flop: true,
                },
                TILES[WIRE] + KL,
            );

            out.fractal.set(
                TilePos {
                    depth: 4,
                    pos: (4, 6).into(),
                    flop: false,
                },
                TILES[SUCKER] + KR,
            );
            out.fractal.set(
                TilePos {
                    depth: 4,
                    pos: (4, 7).into(),
                    flop: false,
                },
                TILES[SPINNER] + KL,
            );
            out.activate(TilePos {
                depth: 4,
                pos: (4, 7).into(),
                flop: false,
            });
            out.fractal.set(
                TilePos {
                    depth: 4,
                    pos: (4, 6).into(),
                    flop: true,
                },
                TILES[WIRE] + KR,
            );
        }
        Config::TestGrowBug => {
            out.fractal.set(
                TilePos {
                    depth: 3,
                    pos: (1, 2).into(),
                    flop: false,
                },
                TILES[GROWER] + KR,
            );
            out.activate(TilePos {
                depth: 3,
                pos: (1, 2).into(),
                flop: false,
            });
            out.fractal.set(
                TilePos {
                    depth: 4,
                    pos: (1, 4).into(),
                    flop: false,
                },
                TILES[X],
            );

            out.fractal.set(
                TilePos {
                    depth: 3,
                    pos: (2, 2).into(),
                    flop: false,
                },
                TILES[GROWER],
            );
            out.activate(TilePos {
                depth: 3,
                pos: (2, 2).into(),
                flop: false,
            });
            out.fractal.set(
                TilePos {
                    depth: 4,
                    pos: (5, 6).into(),
                    flop: false,
                },
                TILES[X],
            );
        }
        Config::TestActiveBug => {
            out.fractal.set(
                TilePos {
                    depth: 4,
                    pos: (2, 5).into(),
                    flop: true,
                },
                TILES[SPINNER],
            );
            out.activate(TilePos {
                depth: 4,
                pos: (2, 5).into(),
                flop: true,
            });

            out.fractal.set(
                TilePos {
                    depth: 4,
                    pos: (3, 5).into(),
                    flop: true,
                },
                TILES[SPINNER] + FU,
            );
            out.activate(TilePos {
                depth: 4,
                pos: (3, 5).into(),
                flop: true,
            });
        }
    }

    out
}
