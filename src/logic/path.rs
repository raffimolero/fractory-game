use std::ops::Mul;
use std::ops::{Add, AddAssign, Neg};

use super::fractal::Tringle;

use glam::IVec2;

/// glam doesn't have one 😠
struct IMat2 {
    x_axis: IVec2,
    y_axis: IVec2,
}

impl IMat2 {
    const fn new([xx, xy]: [i32; 2], [yx, yy]: [i32; 2]) -> Self {
        Self {
            x_axis: IVec2::new(xx, xy),
            y_axis: IVec2::new(yx, yy),
        }
    }
}

impl Mul<IVec2> for IMat2 {
    type Output = IVec2;

    fn mul(self, rhs: IVec2) -> Self::Output {
        self.x_axis * rhs.x + self.y_axis * rhs.y
    }
}

// pub mod flop {
//     pub const DOWN: u8 = 1;
//     pub const LEFT: u8 = 2;
//     pub const RIGHT: u8 = 3;
// }
pub enum SubTile {
    D,
    U,
    R,
    L,
}

/**
```txt
  /\
 /yx\
/____\
\ yxf/
 \  /
  \/

template
\    /\    /\    /\    /\    /\    /\    /\    /\
 \  /  \  /  \  /  \  /  \  /  \  /  \  /  \  /  \
  \/____\/____\/____\/____\/____\/____\/____\/____\
  /\    /\    /\    /\    /\    /\    /\    /\    /
 /  \  /  \  /  \  /  \  /  \  /  \  /  \  /  \  /
/____\/____\/____\/____\/____\/____\/____\/____\/

rotations concerning flips:
            y  x
       cw: -1 -1
       cc: -1  =
            /\
           /  \
          /____\
         /\    /\
        /  \  /  \
       /____\/____\
      /\-1-1/\-1,0/\
     /  \f /00\f /  \
    /____\/____\/____\
   /\    /\=0,0/\    /\
  /  \  /  \f /  \  /  \
 /____\/____\/____\/____\

\    /\    /\    /\    /\    /\    /\    /\    /\
 \  /  \  /  \  /  \  /  \  /  \  /  \  /  \  /  \
  \/____\/____\/____\/____\/____\/____\/____\/____\
  /\    /\    /\-2-2/\    /\    /\    /\    /\    /
 /  \  /  \  /  \f /-1\  /  \  /  \  /  \  /  \  /
/____\/____\/____\/_-1_\/____\/____\/____\/____\/
\    /\    /\    /\    /\    /\-1+1/\    /\    /\
 \  /  \  /  \  /  \  /00\  /=0\f /  \  /  \  /  \
  \/____\/____\/____\/____\/_+1_\/____\/____\/____\
  /\    /\    /\    /\    /\    /\    /\    /\    /
 /  \  /  \  /  \  /+1\  /  \  /  \  /  \  /  \  /
/____\/____\/____\/_=0_\/____\/____\/____\/____\/
\    /\    /\    /\+1,0/\    /\    /\    /\    /\
 \  /  \  /  \  /  \f /  \  /  \  /  \  /  \  /  \
  \/____\/____\/____\/____\/____\/____\/____\/____\
  /\    /\    /\    /\    /\    /\    /\    /\    /
 /  \  /  \  /  \  /  \  /  \  /  \  /  \  /  \  /
/____\/____\/____\/____\/____\/____\/____\/____\/

horizontal reflections concerning the y axis: directly proportional
\    /\    /\    /\    /\    /\    /\    /\    /\
 \  /  \  /  \  /-2\  /  \  /-2\  /  \  /  \  /  \
  \/____\/____\/_-2_\/____\/_=0_\/____\/____\/____\
  /\    /\    /\    /\    /\    /\    /\    /\    /
 /  \  /  \  /  \  /-1\  /-1\  /  \  /  \  /  \  /
/____\/____\/____\/_-1_\/_=0_\/____\/____\/____\/
\    /\    /\    /\    /\    /\    /\    /\    /\
 \  /  \  /  \  /  \  /00\  /  \  /  \  /  \  /  \
  \/____\/____\/____\/____\/____\/____\/____\/____\
  /\    /\    /\    /\    /\    /\    /\    /\    /
 /  \  /  \  /  \  /+1\  /+1\  /  \  /  \  /  \  /
/____\/____\/____\/_=0_\/_+1_\/____\/____\/____\/
\    /\    /\    /\    /\    /\    /\    /\    /\
 \  /  \  /  \  /+2\  /  \  /+2\  /  \  /  \  /  \
  \/____\/____\/_=0_\/____\/_+2_\/____\/____\/____\
  /\    /\    /\+2=0/\    /\+2+2/\    /\    /\    /
 /  \  /  \  /  \f /  \  /  \f /  \  /  \  /  \  /
/____\/____\/____\/____\/____\/____\/____\/____\/

horizontal reflections concerning the x axis: do i have to tell you
\    /\    /\    /\    /\    /\    /\    /\    /\
 \  /  \  /  \  /  \  /  \  /  \  /  \  /  \  /  \
  \/____\/____\/____\/____\/____\/____\/____\/____\
  /\    /\    /\    /\    /\    /\    /\    /\    /
 /  \  /  \  /  \  /  \  /  \  /  \  /  \  /  \  /
/____\/____\/____\/____\/____\/____\/____\/____\/
\    /\    /\    /\    /\    /\    /\    /\    /\
 \  /  \  /-2\  /-1\  /00\  /+1\  /+2\  /  \  /  \
  \/____\/____\/____\/____\/____\/____\/____\/____\
  /\    /\    /\    /\    /\    /\    /\    /\    /
 /  \  /  \  /  \  /  \  /  \  /  \  /  \  /  \  /
/____\/____\/____\/____\/____\/____\/____\/____\/
\    /\    /\    /\    /\    /\    /\    /\    /\
 \  /  \  /  \  /  \  /  \  /  \  /  \  /  \  /  \
  \/____\/____\/____\/____\/____\/____\/____\/____\
  /\    /\    /\    /\    /\    /\    /\    /\    /
 /  \  /  \  /  \  /  \  /  \  /  \  /  \  /  \  /
/____\/____\/____\/____\/____\/____\/____\/____\/

template
\    /\    /\    /\    /\    /\    /\    /\    /\
 \  /  \  /  \  /  \  /  \  /  \  /  \  /  \  /  \
  \/____\/____\/____\/____\/____\/____\/____\/____\
  /\    /\    /\    /\    /\    /\    /\    /\    /
 /  \  /  \  /  \  /  \  /  \  /  \  /  \  /  \  /
/____\/____\/____\/____\/____\/____\/____\/____\/
```
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TileOffset {
    offset: IVec2,
    flop: bool,
}
impl TileOffset {
    fn new(x: i32, y: i32, flop: bool) -> Self {
        Self {
            offset: IVec2 { x, y },
            flop,
        }
    }

    const ROT_CW: IMat2 = IMat2::new([0, 1], [-1, -1]);
    const ROT_CC: IMat2 = IMat2::new([-1, -1], [1, 0]);
    const FLIP: IMat2 = IMat2::new([-1, 0], [1, 1]);

    // Note: these transforms might benefit from an imat3 where the 3rd dimension is just a bool
    // i decided not to do that

    fn flip(&mut self) {
        self.offset = Self::FLIP * self.offset;
    }

    fn rotate_cw(&mut self) {
        self.offset = Self::ROT_CW * self.offset;
        if self.flop {
            self.offset.x -= 1;
            self.offset.y -= 1;
        }
    }
    fn rotate_cc(&mut self) {
        self.offset = Self::ROT_CC * self.offset;
        if self.flop {
            self.offset.y -= 1;
        }
    }
}

impl Neg for TileOffset {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        // self.flop ^= true;?
        self.offset *= -1;
        self
    }
}

impl AddAssign for TileOffset {
    fn add_assign(&mut self, rhs: Self) {
        self.offset += rhs.offset;
        if rhs.flop {
            *self = -*self;
        }
    }
}

impl Add for TileOffset {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

/**
```txt
on upscaling
  /\
 /yx\
/____\
\ yxf/
 \  /
  \/


    U      L      R            D
   /\     /\     /\           /\
  /00\   /10\   /11\         /00\
 /____\ /____\ /____\       /_f__\
   ==     y+h   +h+h    b*2-y b-x flop
                         where b=h-1

     /\            /\            /\            /\
    /00\          /20\          /22\          /21\
   /____\        /____\        /____\        /_f__\
  /\ 00f/\      /\ 20f/\      /\ 22f/\      /\ 21 /\
 /10\  /11\    /30\  /31\    /32\  /33\    /11\  /10\
/____\/____\  /____\/____\  /____\/____\  /_f__\/_f__\
     ==            y+2          +2 +2     2-y 1-x flop


           /\                         /\
          /00\                       /  \
         /____\                     /____\
        /\ 00f/\                   /\    /\
       /10\  /11\                 /  \  /  \
      /____\/____\               /____\/____\
     /\ 10f/\ 11f/\             /\-1-1/\-1,0/\
    /20\  /21\  /22\           /  \f /00\f /  \
   /____\/____\/____\         /____\/____\/____\
  /\ 20f/\ 21f/\ 22f/\       /\    /\=0,0/\    /\
 /30\  /31\  /32\  /33\     /  \  /  \f /  \  /  \
/____\/____\/____\/____\   /____\/____\/____\/____\

           /\                       /\                       /\                       /\
          /00\                     /40\                     /44\                     /63\
         /____\                   /____\                   /____\                   /_f__\
        /\ 00f/\                 /\ 40f/\                 /\ 44f/\                 /\ 63 /\
       /10\  /11\               /50\  /51\               /54\  /55\               /53\  /52\
      /____\/____\             /____\/____\             /____\/____\             /_f__\/_f__\
     /\ 10f/\ 11f/\           /\ 50f/\ 51f/\           /\ 54f/\ 55f/\           /\ 53 /\ 52 /\
    /20\  /21\  /22\         /60\  /61\  /62\         /64\  /65\  /66\         /43\  /42\  /41\
   /____\/____\/____\       /____\/____\/____\       /____\/____\/____\       /_f__\/_f__\/_f__\
  /\ 20f/\ 21f/\ 22f/\     /\ 60f/\ 61f/\ 62f/\     /\ 64f/\ 65f/\ 66f/\     /\ 43 /\ 42 /\ 41 /\
 /30\  /31\  /32\  /33\   /70\  /71\  /72\  /73\   /74\  /75\  /76\  /77\   /33\  /32\  /31\  /30\
/____\/____\/____\/____\ /____\/____\/____\/____\ /____\/____\/____\/____\ /_f__\/_f__\/_f__\/_f__\
           ==                       y+4                     +4 +4                 6-y 3-x flop


                       /\
                      /00\
                     /____\
                    /\ 00f/\
                   /10\  /11\
                  /____\/____\
                 /\ 10f/\ 11f/\
                /20\  /21\  /22\
               /____\/____\/____\
              /\ 20f/\ 21f/\ 22f/\
             /30\  /31\  /32\  /33\
            /____\/____\/____\/____\
           /\ 30f/\ 31f/\ 32f/\ 33f/\
          /40\  /41\  /42\  /43\  /44\
         /____\/____\/____\/____\/____\
        /\ 40f/\ 41f/\ 42f/\ 43f/\ 44f/\
       /50\  /51\  /52\  /53\  /54\  /55\
      /____\/____\/____\/____\/____\/____\
     /\ 50f/\ 51f/\ 52f/\ 53f/\ 54f/\ 55f/\
    /60\  /61\  /62\  /63\  /64\  /65\  /66\
   /____\/____\/____\/____\/____\/____\/____\
  /\ 60f/\ 61f/\ 62f/\ 63f/\ 64f/\ 65f/\ 66f/\
 /70\  /71\  /72\  /73\  /74\  /75\  /76\  /77\
/____\/____\/____\/____\/____\/____\/____\/____\
```
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TilePos {
    depth: usize,
    pos: IVec2,
    flop: bool,
}

impl TilePos {
    pub const ZERO: Self = Self {
        depth: 0,
        pos: IVec2::ZERO,
        flop: false,
    };

    pub fn upscale(&mut self, placement: SubTile) {
        let h = 1 << self.depth;
        self.depth += 1;
        match placement {
            SubTile::D => {
                let b = h - 1;
                self.pos.y = b * 2 - self.pos.y;
                self.pos.x = b - self.pos.x;
                self.flop ^= true;
            }
            SubTile::U => {}
            SubTile::R => {
                self.pos.x += h;
                self.pos.y += h;
            }
            SubTile::L => {
                self.pos.y += h;
            }
        }
    }
}

#[test]
fn test_upscale() {
    let mut pos = TilePos::ZERO;
    pos.upscale(SubTile::L);
    assert_eq!(
        pos,
        TilePos {
            depth: 1,
            pos: IVec2::new(0, 1),
            flop: false,
        }
    );
    pos.upscale(SubTile::U);
    assert_eq!(
        pos,
        TilePos {
            depth: 2,
            pos: IVec2::new(0, 1),
            flop: false,
        }
    );
    pos.upscale(SubTile::D);
    assert_eq!(
        pos,
        TilePos {
            depth: 3,
            pos: IVec2::new(3, 5),
            flop: true,
        }
    );
}

#[test]
fn test_flip() {
    let original = TileOffset::new(0, 2, true);
    let mut temp = original;
    temp.flip();
    assert_eq!(temp, TileOffset::new(2, 2, true));
    temp.flip();
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

// fn rotate(order: u8, p: [i32; 2]) -> [i32; 2] {
//     let size = 1 << order;
//     let max_h = size - 1;
//     let max_w = max_h * 2;

// }

#[test]
fn test() {
    let order = 2;
    for y in 0..1 << order {
        for x in 0..y * 2 + 1 {
            print!("({x},{y}) ");
        }
        println!();
    }
}
