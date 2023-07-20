use std::collections::HashSet;

use super::*;

/// enumerates all possible permutations of subtiles,
/// checks if all positions are unique,
/// and verifies that every operation is reversible
#[test]
fn test_subtiles() {
    fn inner(pos: &mut TilePos, set: &mut HashSet<TilePos>, depth: u8) {
        if depth == 0 {
            return;
        }
        for subtile in SubTile::ORDER {
            let save = *pos;
            pos.push_front(subtile);
            assert!(pos.is_valid());
            assert!(set.insert(*pos));
            println!("{pos:?}");

            inner(pos, set, depth - 1);

            assert_eq!(pos.pop_front(), Some(subtile));
            assert_eq!(*pos, save);
        }
    }

    let mut pos = TilePos::UNIT;
    assert_eq!(pos.pop_front(), None);
    let mut set = HashSet::new();
    inner(&mut pos, &mut set, 5);
    assert_eq!(pos.pop_front(), None);
}

#[test]
fn test_supertile_path() {
    let mut pos = TilePos::UNIT;

    let a = pos;
    pos.push_front(SubTile::L);
    assert_eq!(
        pos,
        TilePos {
            depth: 1,
            pos: IVec2::new(0, 1),
            flop: false,
        }
    );

    let b = pos;
    pos.push_front(SubTile::U);
    assert_eq!(
        pos,
        TilePos {
            depth: 2,
            pos: IVec2::new(0, 1),
            flop: false,
        }
    );

    let c = pos;
    pos.push_front(SubTile::C);
    assert_eq!(
        pos,
        TilePos {
            depth: 3,
            pos: IVec2::new(3, 5),
            flop: true,
        }
    );

    let d = pos;
    pos.push_front(SubTile::R);
    assert_eq!(
        pos,
        TilePos {
            depth: 4,
            pos: IVec2::new(11, 13),
            flop: true,
        }
    );

    assert_eq!(pos.pop_front(), Some(SubTile::R));
    assert_eq!(pos, d);
    assert_eq!(pos.pop_front(), Some(SubTile::C));
    assert_eq!(pos, c);
    assert_eq!(pos.pop_front(), Some(SubTile::U));
    assert_eq!(pos, b);
    assert_eq!(pos.pop_front(), Some(SubTile::L));
    assert_eq!(pos, a);
    assert_eq!(pos.pop_front(), None);
    assert_eq!(pos, a);
    assert_eq!(pos, TilePos::UNIT);
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
