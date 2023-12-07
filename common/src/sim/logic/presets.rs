use std::collections::BTreeMap;

use super::{
    factory::{ActiveTiles, Fractory, FractoryMeta},
    fractal::Fractal,
    orientation::Transform,
    orientation::{Orient, Transform::*},
    path::TilePos,
    planet::{BiomeId, Planet, PlanetCache},
    tile::{Quad, Tile},
};

pub mod tiles {
    pub const SPACE: usize = 0;
    pub const X: usize = 1;
    pub const Y: usize = 2;
    pub const Z: usize = 3;
    pub const W: usize = 4;
    pub const ROTOR: usize = 5;
    pub const GROWER: usize = 6;
    pub const SUCKER: usize = 7;
    pub const WIRE: usize = 8;

    pub const LEAF_COUNT: usize = 9;
}
use tiles::*;

pub const TILES: [Tile; LEAF_COUNT] = {
    use Orient::*;
    const ORIENTS: [Orient; LEAF_COUNT] = [
        Iso, // SPACE
        Iso, // X
        Iso, // Y
        RfU, // Z
        AKU, // W
        RtK, // ROTOR
        RfU, // GROWER
        RfU, // SUCKER
        RfU, // WIRE
    ];

    let mut out = [Tile::SPACE; LEAF_COUNT];
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

// // TODO: FOR TESTING
pub const QUADS: [Quad<Tile>; LEAF_COUNT] = {
    const INDEX_TF: [[(usize, Transform); 4]; LEAF_COUNT] = [
        [(SPACE, KU); 4],                     // SPACE
        [(X, KU), (Y, KU), (Y, KU), (Y, KU)], // X
        [(Y, KU), (X, KU), (X, KU), (X, KU)], // Y
        [(X, KU), (X, KU), (Y, KU), (Y, KU)], // Z
        [(Z, KU), (X, KU), (Y, KU), (X, KU)], // W
        [(X, KU), (Z, KR), (Z, KL), (Z, KU)], // ROTOR
        [(Z, KU), (X, KU), (Y, KU), (Y, KU)], // GROWER
        [(Z, KU), (Y, KU), (X, KU), (X, KU)], // SUCKER
        [(Y, KU), (Y, KU), (X, KU), (X, KU)], // WIRE
    ];

    let mut out = [Quad::SPACE; LEAF_COUNT];
    let mut i = 0;
    while i < LEAF_COUNT {
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

// pub const QUADS: [Quad; LEAF_COUNT] = [
//     Quad([TILES[SPACE]; 4]),                        // SPACE
//     Quad([TILES[X], TILES[Y], TILES[Y], TILES[Y]]), // X
//     Quad([TILES[Y], TILES[X], TILES[X], TILES[X]]), // Y
//     Quad([TILES[X], TILES[X], TILES[Y], TILES[Y]]), // Z
//     Quad([TILES[Z], TILES[X], TILES[Y], TILES[X]]), // W
//     Quad([
//         TILES[X],
//         Tile {
//             id: TILES[Z].id,
//             orient: TILES[Z].orient.rot_cw(),
//         },
//         Tile {
//             id: TILES[Z].id,
//             orient: TILES[Z].orient.rot_cw().rot_cw(),
//         },
//         TILES[Z],
//     ]), // ROTOR
//     Quad([TILES[Z], TILES[X], TILES[Y], TILES[Y]]), // GROWER
//     Quad([TILES[Z], TILES[Y], TILES[X], TILES[X]]), // SUCKER
//     Quad([TILES[Y], TILES[Y], TILES[X], TILES[X]]), // WIRE
// ];

pub fn new_xyyy_fractal() -> Fractal {
    // TODO: take fragments as argument
    Fractal::new(&[
        QUADS[X],
        QUADS[Y],
        QUADS[Z],
        QUADS[W],
        QUADS[ROTOR],
        QUADS[GROWER],
        QUADS[SUCKER],
        QUADS[WIRE],
    ])
    .unwrap()
}

pub fn new_xyyy_fractory_meta(planets: &mut PlanetCache) -> FractoryMeta {
    let xyyy = Planet::new_xyyy();
    let planet_id = xyyy.default_id();
    // let biome_id = BiomeId::from("Spinless");
    let biome_id = BiomeId::from("Landing Zone");
    planets.register(planet_id.clone(), xyyy);
    FractoryMeta {
        fractory: new_xyyy_fractory(),
        planet: planet_id,
        biome: biome_id,
    }
}

pub fn new_xyyy_fractory() -> Fractory {
    let mut out = Fractory {
        fractal: new_xyyy_fractal(),
        activated: ActiveTiles::new(),
        inventory: BTreeMap::new(),
    };

    out.fractal.set(TilePos::UNIT, Tile::SPACE);

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
                TILES[Z],
            );

            out.fractal.set(
                TilePos {
                    depth: 1,
                    pos: (0, 0).into(),
                    flop: true,
                },
                TILES[Z] + KR,
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
                TILES[Z],
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
                TILES[Z],
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
                TILES[W],
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
                TILES[W],
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
                Tile {
                    id: TILES[W].id,
                    orient: TILES[W].orient.rot_cw(),
                },
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
                TILES[W],
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
                Tile {
                    id: TILES[W].id,
                    orient: TILES[W].orient.rot_cw(),
                },
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
                Tile {
                    id: TILES[W].id,
                    orient: TILES[W].orient.rot_cw().rot_cw(),
                },
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
                TILES[W],
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
                TILES[W] + FU,
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
