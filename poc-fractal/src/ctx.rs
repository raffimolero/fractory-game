use std::time::Instant;

use super::*;

#[derive(Debug, Default)]
pub struct Context {
    mouse: Option<Vec2>,
    last_lmb: Option<(Vec2, Instant)>,
    last_rmb: Option<(Vec2, Instant)>,
    matrix: Mat4,
    inv_matrix: Mat4,
}

impl Context {
    pub fn apply(&mut self, matrix: Mat4, f: impl FnOnce(&mut Self)) {
        let orig = (self.matrix, self.inv_matrix);
        self.matrix = self.matrix * matrix;
        self.inv_matrix = self.matrix.inverse();
        ergoquad_2d::prelude::apply(matrix, || f(self));
        (self.matrix, self.inv_matrix) = orig;
    }

    pub fn draw_canvas(&mut self, matrix: Mat4, paint: impl FnOnce(&mut Self)) {
        todo!("reset model matrix, make canvas with computed size");
    }

    pub fn project(&self, point: Vec2) -> Vec2 {
        self.inv_matrix
            .transform_point3(point.extend(0.0))
            .truncate()
    }

    pub fn mouse_pos(&self) -> Option<Vec2> {
        self.mouse.map(|pos| self.project(pos))
    }

    pub fn lmb_pos(&self) -> Option<(Vec2, Instant)> {
        self.last_lmb.map(|(pos, time)| (self.project(pos), time))
    }

    pub fn rmb_pos(&self) -> Option<(Vec2, Instant)> {
        self.last_rmb.map(|(pos, time)| (self.project(pos), time))
    }

    pub fn update(&mut self) {
        self.mouse.replace(mouse_position_local());
        let now = Instant::now();
        if is_mouse_button_pressed(MouseButton::Left) {
            self.last_lmb = self.mouse.map(|pos| (pos, now));
        }
        if is_mouse_button_pressed(MouseButton::Right) {
            self.last_rmb = self.mouse.map(|pos| (pos, now));
        }
    }
}
pub struct Click {
    pub pos: Vec2,
    pub held: bool,
}
impl Context {
    pub fn get_click(&self) -> Option<Click> {
        const SCREEN_WIDTH: f32 = 2.0;
        const LEASH_RANGE: f32 = SCREEN_WIDTH / 4.0;
        const HOLD_DURATION: Duration = Duration::from_secs(1);

        let (down, lmb_time) = self.lmb_pos()?;
        let up = self.mouse_pos()?;

        let leash_sq = LEASH_RANGE * LEASH_RANGE;
        let in_range = (down - up).length_squared() < leash_sq;

        let hold_time = Instant::now() - lmb_time;
        let held = hold_time >= HOLD_DURATION;

        in_range.then(|| Click { pos: down, held })
    }
}
