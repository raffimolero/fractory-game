use macroquad::prelude::*;

pub trait Clickable {
    fn try_click(&mut self, mouse_pos: Vec2);
}
