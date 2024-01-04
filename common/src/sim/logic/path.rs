#[cfg(test)]
mod tests;

use super::{
    orientation::{Rotation, Transform},
    tile::SubTile,
};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul},
    str::{Chars, FromStr},
};

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
    pub depth: u8,
    pub offset: IVec2,
    pub flop: bool,
}

impl TileOffset {
    pub const ZERO: Self = Self {
        depth: 0,
        offset: IVec2 { x: 0, y: 0 },
        flop: false,
    };

    // Note: these transforms might benefit from an imat3 where the 3rd dimension is just a bool
    // i decided not to do that
    const FLIP_X: IMat2 = IMat2::new([-1, 0], [1, 1]);
    const ROT_CW: IMat2 = IMat2::new([0, 1], [-1, -1]);
    const ROT_CC: IMat2 = IMat2::new([-1, -1], [1, 0]);

    pub fn flip_x(&mut self) {
        self.offset = Self::FLIP_X * self.offset;
    }

    fn c_off(self) -> i32 {
        (1 << self.depth) - 1 - (self.flop as i32)
    }

    pub fn rotate_cw(&mut self) {
        self.offset = Self::ROT_CW * self.offset;
        let c_off = self.c_off();
        self.offset.x += c_off;
        self.offset.y += c_off;
    }

    pub fn rotate_cc(&mut self) {
        self.offset = Self::ROT_CC * self.offset;
        let c_off = self.c_off();
        self.offset.y += c_off;
    }
}

impl AddAssign<Transform> for TileOffset {
    fn add_assign(&mut self, rhs: Transform) {
        if rhs.reflected() {
            self.flip_x();
        }
        match rhs.rotation() {
            Rotation::U => {}
            Rotation::R => self.rotate_cw(),
            Rotation::L => self.rotate_cc(),
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


    U      L      R            C
   /\     /\     /\           /\
  /00\   /10\   /11\         /00\
 /____\ /____\ /____\       /_f__\
   ==     y+h   +h+h    b*2-y b-x flop      <== push front
                         where b=h-1

 x*2+f  U,y+f  L,x+f        U flop          <== push back
 y*2+2f                                        let f = if base.flop { -1 } else { 1 }

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
/// Functions like a VecDeque<SubTile> with its push/pop methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TilePos {
    pub depth: u8,
    pub pos: IVec2,
    pub flop: bool,
}

impl Display for TilePos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "d{}x{}y{}{}",
            self.depth,
            self.pos.x,
            self.pos.y,
            if self.flop { "f" } else { "" }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TilePosErr {
    UnexpectedToken,
    UnexpectedEndOfString,
    OutOfBounds,
}

impl FromStr for TilePos {
    type Err = TilePosErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        match chars.next() {
            Some('d') => {}
            Some(_) => return Err(TilePosErr::UnexpectedToken),
            None => return Err(TilePosErr::UnexpectedEndOfString),
        };

        let mut depth: u8 = 0;
        loop {
            let Some(c) = chars.next() else {
                return Err(TilePosErr::UnexpectedEndOfString);
            };
            if c == 'x' {
                break;
            }
            depth = depth.checked_mul(10).ok_or(TilePosErr::OutOfBounds)?;
            let d = c.to_digit(10).ok_or(TilePosErr::UnexpectedToken)? as u8;
            depth = depth.checked_add(d).ok_or(TilePosErr::OutOfBounds)?;
        }

        let mut x: i32 = 0;
        loop {
            let Some(c) = chars.next() else {
                return Err(TilePosErr::UnexpectedEndOfString);
            };
            if c == 'y' {
                break;
            }
            x = x.checked_mul(10).ok_or(TilePosErr::OutOfBounds)?;
            let d = c.to_digit(10).ok_or(TilePosErr::UnexpectedToken)? as i32;
            x = x.checked_add(d).ok_or(TilePosErr::OutOfBounds)?;
        }

        let mut y: i32 = 0;
        let flop = loop {
            let Some(c) = chars.next() else {
                break false;
            };
            if c == 'f' {
                if chars.next().is_some() {
                    return Err(TilePosErr::UnexpectedToken);
                }
                break true;
            }
            y = y.checked_mul(10).ok_or(TilePosErr::OutOfBounds)?;
            let d = c.to_digit(10).ok_or(TilePosErr::UnexpectedToken)? as i32;
            y = y.checked_add(d).ok_or(TilePosErr::OutOfBounds)?;
        };

        let out = Self {
            depth,
            pos: IVec2 { x, y },
            flop,
        };
        if !out.is_valid() {
            return Err(TilePosErr::OutOfBounds);
        }
        Ok(out)
    }
}

impl TilePos {
    pub const UNIT: Self = Self {
        depth: 0,
        pos: IVec2::ZERO,
        flop: false,
    };

    pub fn from_inward_path(path_iter: impl IntoIterator<Item = SubTile>) -> Self {
        let mut out = Self::UNIT;
        for subtile in path_iter {
            out.push_inner(subtile);
        }
        out
    }

    pub fn from_outward_path(path_iter: impl IntoIterator<Item = SubTile>) -> Self {
        let mut out = Self::UNIT;
        for subtile in path_iter {
            out.push_outer(subtile);
        }
        out
    }

    pub fn depth(self) -> usize {
        self.depth as usize
    }

    pub fn height(self) -> i32 {
        1 << self.depth
    }

    fn row(self) -> i32 {
        self.pos.y + self.flop as i32
    }

    pub fn is_valid(self) -> bool {
        // depth must be reasonable
        self.depth <= 30
            // x must not be negative
            && self.pos.x >= 0
            // x must fit within the row
            && self.pos.x <= self.pos.y
            // y must not go beneath its height
            && self.row() < self.height()
    }

    #[doc(alias = "push_front")]
    pub fn push_outer(&mut self, placement: SubTile) {
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
            SubTile::R => self.pos += IVec2::splat(h),
            SubTile::L => self.pos.y += h,
        }
    }

    #[doc(alias = "push_back")]
    pub fn push_inner(&mut self, placement: SubTile) {
        self.depth += 1;
        self.pos *= 2;
        let f = if self.flop {
            self.pos += IVec2 { x: 1, y: 2 };
            -1
        } else {
            1
        };

        match placement {
            SubTile::C => self.flop ^= true,
            SubTile::U => {}
            SubTile::R => self.pos += IVec2::splat(f),
            SubTile::L => self.pos.y += f,
        }
    }

    #[doc(alias = "pop_front")]
    pub fn pop_outer(&mut self) -> Option<SubTile> {
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

    #[doc(alias = "pop_back")]
    pub fn pop_inner(&mut self) -> Option<SubTile> {
        self.depth = self.depth.checked_sub(1)?;

        let IVec2 { x, y } = self.pos % 2;
        self.pos /= 2;

        let parity = y << 1 | x;

        if parity == 0b_01 {
            self.pos.y -= 1;
        }

        Some(match parity ^ self.flop as i32 {
            0b_00 => {
                self.flop ^= true;
                SubTile::C
            }
            0b_01 => SubTile::U,
            0b_10 => SubTile::R,
            0b_11 => SubTile::L,
            _ => unreachable!(),
        })
    }
}

impl Add<SubTile> for TilePos {
    type Output = Self;

    fn add(mut self, rhs: SubTile) -> Self::Output {
        self.push_inner(rhs);
        self
    }
}

impl Mul<SubTile> for TilePos {
    type Output = Self;

    fn mul(mut self, rhs: SubTile) -> Self::Output {
        self.push_outer(rhs);
        self
    }
}

impl Add<TileOffset> for TilePos {
    type Output = Option<Self>;

    /// returns None if out of bounds.
    fn add(mut self, mut rhs: TileOffset) -> Self::Output {
        for _ in 0..rhs.depth {
            self.push_inner(SubTile::U);
        }
        if self.flop {
            rhs.offset *= -1;
        }
        self.pos += rhs.offset;
        self.flop ^= rhs.flop;
        self.is_valid().then_some(self)
    }
}

impl Iterator for TilePos {
    type Item = SubTile;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_outer()
    }
}

impl DoubleEndedIterator for TilePos {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.pop_inner()
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
