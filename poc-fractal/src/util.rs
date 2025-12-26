use crate::prelude::*;

pub const SCREEN_SPACE_WIDTH: f32 = 2.0;
pub const SCREEN_SPACE_HEIGHT: f32 = 2.0;

pub fn rect_to_points(rect: Rect) -> [Vec2; 4] {
    let x = rect.x;
    let y = rect.y;
    let r = rect.right();
    let b = rect.bottom();
    [vec2(x, y), vec2(r, y), vec2(r, b), vec2(x, b)]
}

pub fn snap_f32(value: f32, snap_modulus: f32) -> f32 {
    (value / snap_modulus).round() * snap_modulus
}

pub fn dist_to_snap_f32(value: f32, snap_modulus: f32) -> f32 {
    dbg!(snap_f32(value, snap_modulus)) - dbg!(value)
}
