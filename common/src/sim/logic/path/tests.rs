use std::collections::HashSet;

use super::*;

#[test]
fn test_parse_tilepos() {
    for (i, s) in [
        "d0x0y0",
        "d1x0y0f",
        "d1x0y0",
        "d30x1073741823y1073741823",
        "d30x1073741822y1073741822f",
    ]
    .into_iter()
    .enumerate()
    {
        dbg!(i);
        assert_eq!(s, s.parse::<TilePos>().unwrap().to_string());
    }

    for (i, (s, e)) in [
        ("", TilePosErr::UnexpectedEndOfString),
        ("d", TilePosErr::UnexpectedEndOfString),
        ("a", TilePosErr::UnexpectedToken),
        ("d0x0y0h", TilePosErr::UnexpectedToken),
        ("d1x0y0fh", TilePosErr::UnexpectedToken),
        ("d0x0y0f", TilePosErr::OutOfBounds),
        ("d0x1y0", TilePosErr::OutOfBounds),
        ("d0x0y1", TilePosErr::OutOfBounds),
        ("d1x1y0", TilePosErr::OutOfBounds),
        ("d1x1y0f", TilePosErr::OutOfBounds),
        ("d2x2y1", TilePosErr::OutOfBounds),
        ("d31x0y0", TilePosErr::OutOfBounds),
        ("d30x1073741824y0", TilePosErr::OutOfBounds),
        ("d30x0y1073741824", TilePosErr::OutOfBounds),
        ("d30x0y1073741823f", TilePosErr::OutOfBounds),
        ("d30x1073741823y1073741822f", TilePosErr::OutOfBounds),
    ]
    .into_iter()
    .enumerate()
    {
        dbg!(i);
        assert_eq!(s.parse::<TilePos>().unwrap_err(), e);
    }
}

/// enumerates all possible permutations of subtiles,
/// checks if all positions are unique,
/// and verifies that every (push/pop)_(front/back) operation works
#[test]
fn test_subtiles() {
    fn inner(
        pos: &mut TilePos,
        set_front: &mut HashSet<TilePos>,
        set_back: &mut HashSet<TilePos>,
        depth: u8,
    ) {
        if depth == 0 {
            return;
        }
        for subtile in SubTile::QUAD.0 {
            let save = *pos;
            pos.push_outer(subtile);
            assert!(pos.is_valid());
            assert!(set_front.insert(*pos));
            assert!(set_back.insert(*pos));
            println!("{pos:?}");

            inner(pos, set_front, set_back, depth - 1);

            assert_eq!(pos.pop_outer(), Some(subtile));
            assert_eq!(*pos, save);
        }
    }

    let mut pos = TilePos::UNIT;
    assert_eq!(pos.pop_outer(), None);
    assert_eq!(pos.pop_inner(), None);
    let mut set_front = HashSet::new();
    let mut set_back = HashSet::new();
    inner(&mut pos, &mut set_front, &mut set_back, 5);
    assert_eq!(pos.pop_outer(), None);
    assert_eq!(pos.pop_inner(), None);
}

#[test]
fn test_supertile_path() {
    let mut pos = TilePos::UNIT;

    let a = pos;
    pos.push_outer(SubTile::L);
    assert_eq!(
        pos,
        TilePos {
            depth: 1,
            pos: IVec2::new(0, 1),
            flop: false,
        }
    );

    let b = pos;
    pos.push_outer(SubTile::U);
    assert_eq!(
        pos,
        TilePos {
            depth: 2,
            pos: IVec2::new(0, 1),
            flop: false,
        }
    );

    let c = pos;
    pos.push_outer(SubTile::C);
    assert_eq!(
        pos,
        TilePos {
            depth: 3,
            pos: IVec2::new(3, 5),
            flop: true,
        }
    );

    let d = pos;
    pos.push_outer(SubTile::R);
    assert_eq!(
        pos,
        TilePos {
            depth: 4,
            pos: IVec2::new(11, 13),
            flop: true,
        }
    );

    assert_eq!(pos.pop_outer(), Some(SubTile::R));
    assert_eq!(pos, d);
    assert_eq!(pos.pop_outer(), Some(SubTile::C));
    assert_eq!(pos, c);
    assert_eq!(pos.pop_outer(), Some(SubTile::U));
    assert_eq!(pos, b);
    assert_eq!(pos.pop_outer(), Some(SubTile::L));
    assert_eq!(pos, a);
    assert_eq!(pos.pop_outer(), None);
    assert_eq!(pos, a);
    assert_eq!(pos, TilePos::UNIT);
}

#[test]
fn test_flip() {
    let original = TileOffset {
        depth: 0,
        offset: IVec2 { x: 0, y: 2 },
        flop: true,
    };
    let mut temp = original;
    temp.flip_x();
    assert_eq!(
        temp,
        TileOffset {
            depth: 0,
            offset: IVec2 { x: 2, y: 2 },
            flop: true
        }
    );
    temp.flip_x();
    assert_eq!(temp, original);
}

#[test]
fn test_rotate_identities() {
    let mut temp = TileOffset {
        depth: 0,
        offset: IVec2 { x: 15, y: 27 },
        flop: true,
    };
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
