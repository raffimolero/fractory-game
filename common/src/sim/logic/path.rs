// NOTE: quick confession i am absolutely terrified of using these functions in larger scales
// i don't know if they're reliable or anything

#[cfg(test)]
mod tests;

use std::ops::AddAssign;
use std::ops::Mul;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SubTile {
    C, // Center/Core
    U, // Up
    R, // Right
    L, // Left
}

impl SubTile {
    const ORDER: [Self; 4] = [Self::C, Self::U, Self::R, Self::L];
}

/*
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

\    /\    /\    /\    /\    /\    /\
 \  /  \  /  \  /  \  /  \  /  \  /  \
  \/____\/____\/____\/____\/____\/____\
  /\    /\-2-2/\    /\    /\    /\    /
 /  \  /  \f /-1\  /  \  /  \  /  \  /
/____\/____\/_-1_\/____\/____\/____\/
\    /\    /\    /\    /\-1+1/\    /\
 \  /  \  /  \  /00\  /=0\f /  \  /  \
  \/____\/____\/____\/_+1_\/____\/____\
  /\    /\    /\    /\    /\    /\    /
 /  \  /  \  /+1\  /  \  /  \  /  \  /
/____\/____\/_=0_\/____\/____\/____\/
\    /\    /\+1,0/\    /\    /\    /\
 \  /  \  /  \f /  \  /  \  /  \  /  \
  \/____\/____\/____\/____\/____\/____\
  /\    /\    /\    /\    /\    /\    /
 /  \  /  \  /  \  /  \  /  \  /  \  /
/____\/____\/____\/____\/____\/____\/

horizontal reflections concerning the y axis: directly proportional
\    /\    /\    /\    /\    /\
 \  /  \  /-2\  /  \  /-2\  /  \
  \/____\/_-2_\/____\/_=0_\/____\
  /\    /\    /\    /\    /\    /
 /  \  /  \  /-1\  /-1\  /  \  /
/____\/____\/_-1_\/_=0_\/____\/
\    /\    /\    /\    /\    /\
 \  /  \  /  \  /00\  /  \  /  \
  \/____\/____\/____\/____\/____\
  /\    /\    /\    /\    /\    /
 /  \  /  \  /+1\  /+1\  /  \  /
/____\/____\/_=0_\/_+1_\/____\/
\    /\    /\    /\    /\    /\
 \  /  \  /+2\  /  \  /+2\  /  \
  \/____\/_=0_\/____\/_+2_\/____\
  /\    /\+2=0/\    /\+2+2/\    /
 /  \  /  \f /  \  /  \f /  \  /
/____\/____\/____\/____\/____\/

horizontal reflections concerning the x axis: do i have to tell you
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
 /  \  /  \  /  \  /  \  /-1\  /  \  /  \  /  \  /
/____\/____\/____\/____\/_=0_\/____\/____\/____\/
\    /\    /\    /\    /\    /\    /\    /\    /\
 \  /  \  /-2\  /  \  /00\  /+1\  /+2\  /  \  /  \
  \/____\/____\/____\/____\/____\/____\/____\/____\
  /\    /\ -2f/\ -1f/\ 00f/\    /\ +2f/\    /\    /
 /  \  /  \  /  \  /  \  /  \  /  \  /  \  /  \  /
/____\/____\/____\/____\/____\/____\/____\/____\/
\    /\    /\    /\+1=0/\    /\    /\    /\    /\
 \  /  \  /  \  /  \f /  \  /  \  /  \  /  \  /  \
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

/// An offset that can be added to a TilePos, or rotated and reflected.
///
/// Can only move within the same level or deeper, not higher.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TileOffset {
    depth: u8,
    offset: IVec2,
    flop: bool,
}
impl TileOffset {
    fn new(x: i32, y: i32, flop: bool) -> Self {
        Self {
            depth: 0,
            offset: IVec2 { x, y },
            flop,
        }
    }

    // Note: these transforms might benefit from an imat3 where the 3rd dimension is just a bool
    // i decided not to do that

    const FLIP_X: IMat2 = IMat2::new([-1, 0], [1, 1]);
    const ROT_CW: IMat2 = IMat2::new([0, 1], [-1, -1]);
    const ROT_CC: IMat2 = IMat2::new([-1, -1], [1, 0]);

    fn flip_x(&mut self) {
        self.offset = Self::FLIP_X * self.offset;
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

/*
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

/// Locates a specific triangle inside of a fractal.
///
/// Functions like a Vec<SubTile> with its push/pop methods.
/// Can only iterate in reverse; from broad to narrow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TilePos {
    depth: u8,
    pos: IVec2,
    flop: bool,
}

impl TilePos {
    pub const UNIT: Self = Self {
        depth: 0,
        pos: IVec2::ZERO,
        flop: false,
    };

    pub fn from_inward_path(path: &[SubTile]) -> Self {
        let mut out = Self::UNIT;
        for subtile in path.iter().rev().copied() {
            out.push_front(subtile);
        }
        out
    }

    pub fn height(self) -> i32 {
        1 << self.depth
    }

    fn row(self) -> i32 {
        self.pos.y + self.flop as i32
    }

    pub fn is_valid(self) -> bool {
        // must not be negative
        self.pos.x >= 0
            // must fit within the row
            && self.pos.x <= self.pos.y
            // must not go beneath its height
            && self.row() < self.height()
    }

    pub fn push_front(&mut self, placement: SubTile) {
        let h = self.height();
        self.depth += 1;
        match placement {
            SubTile::C => {
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

    pub fn pop_front(&mut self) -> Option<SubTile> {
        self.depth = self.depth.checked_sub(1)?;

        let h = self.height();

        if self.row() < h {
            return Some(SubTile::U);
        }

        self.pos.y -= h;
        if self.pos.x <= self.pos.y {
            return Some(SubTile::L);
        }

        if self.pos.x >= h {
            self.pos.x -= h;
            return Some(SubTile::R);
        }

        self.pos.y = h - 2 - self.pos.y;
        self.pos.x = h - 1 - self.pos.x;
        self.flop ^= true;
        Some(SubTile::C)
    }
}

impl AddAssign<TileOffset> for TilePos {
    fn add_assign(&mut self, mut rhs: TileOffset) {
        for _ in 0..rhs.depth {
            self.push_front(SubTile::C);
        }
        if self.flop {
            rhs.offset *= -1;
        }
        self.pos += rhs.offset;
        self.flop ^= rhs.flop;
        assert!(self.is_valid());
    }
}

impl Iterator for TilePos {
    type Item = SubTile;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_front()
    }
}

// TODO: support other topologies
// /// an absolute position in a quadtree
// trait Position: Default + AddAssign<Self::Relative> {
//     /// an offset, which can be added to get a new position in a quadtree
//     type Relative;

//     /// narrows the scope of this position one layer deeper
//     fn push(&mut self, placement: SubTile);

//     /// broadens the scope of this position one layer shallower
//     fn pop(&mut self) -> Option<SubTile>;

//     /// returns whether the position is valid and within bounds
//     fn is_valid(&self) -> bool;
// }

// /// a form of topology
// trait Topology {
//     /// an absolute position in a quadtree
//     type Absolute: Position;

//     /// how to transform a parent tile into a smaller subtile
//     fn transform(subtile: SubTile) -> Mat4;
// }
