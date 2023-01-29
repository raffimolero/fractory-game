use macroquad::prelude::*;

pub fn flip_x() -> Mat4 {
    Mat4::from_scale(vec3(-1.0, 1.0, 1.0))
}
pub fn flip_y() -> Mat4 {
    Mat4::from_scale(vec3(1.0, -1.0, 1.0))
}
pub fn flip_xy() -> Mat4 {
    Mat4::from_scale(vec3(-1.0, -1.0, 1.0))
}

pub fn scale(x: f32, y: f32) -> Mat4 {
    Mat4::from_scale(vec3(x, y, 1.0))
}
pub fn upscale(factor: f32) -> Mat4 {
    scale(factor, factor)
}
pub fn downscale(factor: f32) -> Mat4 {
    upscale(1.0 / factor)
}

pub fn shift(x: f32, y: f32) -> Mat4 {
    Mat4::from_translation(vec3(x, y, 0.0))
}
pub fn rotate_cw(radians: f32) -> Mat4 {
    Mat4::from_rotation_z(radians)
}
pub fn rotate_cc(radians: f32) -> Mat4 {
    rotate_cw(-radians)
}
