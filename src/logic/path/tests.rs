use std::collections::{HashMap, HashSet};

use super::*;

// less prone to errors but can't be refactored
// macro_rules! _test_subtiles {
//     ($pos:ident, $set:ident, ) => {};

//     ($pos:ident, $set:ident, $_:tt $($t:tt)*) => {
//         for subtile in SubTile::ORDER {
//             let save = $pos;
//             $pos.in_supertile(subtile);
//             assert!($set.insert($pos));
//             println!("{:?}", $pos);

//             _test_subtiles!($pos, $set, $($t)*);

//             assert_eq!($pos.in_subtile(), subtile);
//             assert_eq!($pos, save);
//         }
//     };
// }

/// enumerates all possible permutations of subtiles,
/// checks if all positions are unique,
/// and verifies that every operation is reversible
#[test]
fn test_subtiles() {
    let mut pos = TilePos::UNIT;
    let mut set = HashSet::new();
    for a in SubTile::ORDER {
        let aa = pos;
        pos.promote(a);
        assert!(set.insert(pos));
        println!("{pos:?}");
        for b in SubTile::ORDER {
            let bb = pos;
            pos.promote(b);
            assert!(set.insert(pos));
            println!("{pos:?}");
            for c in SubTile::ORDER {
                let cc = pos;
                pos.promote(c);
                assert!(set.insert(pos));
                println!("{pos:?}");
                for d in SubTile::ORDER {
                    let dd = pos;
                    pos.promote(d);
                    assert!(set.insert(pos));
                    println!("{pos:?}");
                    for e in SubTile::ORDER {
                        let ee = pos;
                        pos.promote(e);

                        assert!(set.insert(pos));
                        println!("{pos:?}");

                        assert_eq!(pos.demote(), e);
                        assert_eq!(pos, ee);
                    }
                    assert_eq!(pos.demote(), d);
                    assert_eq!(pos, dd);
                }
                assert_eq!(pos.demote(), c);
                assert_eq!(pos, cc);
            }
            assert_eq!(pos.demote(), b);
            assert_eq!(pos, bb);
        }
        assert_eq!(pos.demote(), a);
        assert_eq!(pos, aa);
    }
}

#[test]
fn test_supertile_path() {
    let mut pos = TilePos::UNIT;

    let a = pos;
    pos.promote(SubTile::L);
    assert_eq!(
        pos,
        TilePos {
            depth: 1,
            pos: IVec2::new(0, 1),
            flop: false,
        }
    );

    let b = pos;
    pos.promote(SubTile::U);
    assert_eq!(
        pos,
        TilePos {
            depth: 2,
            pos: IVec2::new(0, 1),
            flop: false,
        }
    );

    let c = pos;
    pos.promote(SubTile::C);
    assert_eq!(
        pos,
        TilePos {
            depth: 3,
            pos: IVec2::new(3, 5),
            flop: true,
        }
    );

    let d = pos;
    pos.promote(SubTile::R);
    assert_eq!(
        pos,
        TilePos {
            depth: 4,
            pos: IVec2::new(11, 13),
            flop: true,
        }
    );

    assert_eq!(pos.demote(), SubTile::R);
    assert_eq!(pos, d);
    assert_eq!(pos.demote(), SubTile::C);
    assert_eq!(pos, c);
    assert_eq!(pos.demote(), SubTile::U);
    assert_eq!(pos, b);
    assert_eq!(pos.demote(), SubTile::L);
    assert_eq!(pos, a);
}

#[test]
fn test_flip() {
    let original = TileOffset::new(0, 2, true);
    let mut temp = original;
    temp.flip_x();
    assert_eq!(temp, TileOffset::new(2, 2, true));
    temp.flip_x();
    assert_eq!(temp, original);
}

#[test]
fn test_rotate_identities() {
    let mut temp = TileOffset::new(15, 27, true);
    let a = temp;

    temp.rotate_cc();
    let b = temp;
    assert_ne!(a, b);

    temp.rotate_cc();
    let c = temp;
    assert_ne!(b, c);

    temp.rotate_cc();
    let a2 = temp;
    assert_eq!(a2, a);

    temp.rotate_cw();
    let c2 = temp;
    assert_eq!(c2, c);

    temp.rotate_cw();
    let b2 = temp;
    assert_eq!(b2, b);

    temp.rotate_cw();
    let a3 = temp;
    assert_eq!(a3, a);
}
