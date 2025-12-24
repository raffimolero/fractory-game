use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct FractalCam {
    pub(crate) camera: Mat4,
    pub(crate) min_depth: f32,
    pub(crate) min_bg_depth: f32,
    pub(crate) mouse_depth: f32,
    pub(crate) max_bg_depth: f32,
    pub(crate) max_mouse_depth: f32,
}

impl Default for FractalCam {
    fn default() -> Self {
        Self {
            camera: Mat4::IDENTITY,
            min_depth: -1.0,
            min_bg_depth: 1.0,
            mouse_depth: 4.0,
            max_bg_depth: 4.0,
            max_mouse_depth: 6.0,
        }
    }
}

impl FractalCam {
    /// transforms this camera based on user input
    pub fn input(&mut self, ctx: &Context) {
        use KeyCode::*;

        let [mut x, mut y, mut rot] = [0.0; 3];
        let mut flipped = false;
        let mut zoom = 1.0;
        let mut min_bg_depth = 0.0;
        let mut mouse_depth = 0.0;

        // // nearly every macroquad function uses f32 instead of f64 because that's what `Mat4`s are made of
        // let time = get_time() as f32;
        // for some reason this uses f32s already
        let delta = get_frame_time();

        // check mouse
        // mouse goes downwards, while transforms go upwards
        // if the mouse hasn't moved since startup, this will be 0, 0
        let mut mouse = mouse_position_local();
        // if mouse out of bounds, default to center of screen
        if !(-1.0..1.0).contains(&mouse.x) || !(-1.0..1.0).contains(&mouse.y) {
            mouse = Vec2::ZERO;
        }

        let mouse_zoom = {
            mouse = ctx.project(mouse);
            let mouse_delta = ctx.project(-mouse_delta_position());
            // scroll goes up, transforms zoom in
            let (_scroll_x, scroll_y) = mouse_wheel();

            // drag controls
            if is_mouse_button_down(MouseButton::Middle) {
                x += mouse_delta.x;
                y += mouse_delta.y;
            }

            // zoom
            let scroll_sens = 1.0 / 120.0;
            let zoom_amount = scroll_y * scroll_sens;
            zoom_amount
        };

        // check keypresses
        let keyboard_zoom = if is_key_down(LeftShift) {
            let speed = 4.0;
            // WASD movement, y goes down
            if is_key_down(W) {
                y += delta * speed;
            }
            if is_key_down(S) {
                y -= delta * speed;
            }
            if is_key_down(A) {
                x += delta * speed;
            }
            if is_key_down(D) {
                x -= delta * speed;
            }

            // rotation, clockwise
            let sensitivity = TAU / 2.0; // no i will not use pi
            if is_key_down(Q) {
                rot -= delta * sensitivity;
            }
            if is_key_down(E) {
                rot += delta * sensitivity;
            }
            if is_key_pressed(F) {
                flipped ^= true;
            }
            if is_key_pressed(X) {
                let (_scale, rotation, _translation) = self.camera.to_scale_rotation_translation();
                let (_x, _y, z) = rotation.to_euler(EulerRot::XYZ);
                let snap = TAU / 6.0;
                let offset = snap / 2.0;
                rot -= (z + offset) % snap - offset;
            }

            // zoom
            let zoom_sens = 4.0;
            let mut zoom = 0.0;
            if is_key_down(Z) {
                zoom -= delta * zoom_sens;
            }
            if is_key_down(C) {
                zoom += delta * zoom_sens;
            }
            zoom
        } else {
            0.0
        };

        let zoom_amount = mouse_zoom + keyboard_zoom;
        let mut zoom_scaling = (2_f32).powf(zoom_amount);
        if is_key_down(LeftControl) {
            mouse_depth += zoom_amount;
        } else if is_key_down(LeftAlt) {
            min_bg_depth += zoom_amount;
        } else {
            mouse_depth -= zoom_amount;
            zoom *= zoom_scaling;
        }

        let main_transform = Mat4::from_scale_rotation_translation(
            Vec3 {
                x: if flipped { -zoom } else { zoom },
                y: zoom,
                z: 1.0,
            },
            Quat::from_rotation_z(rot),
            Vec3 { x, y, z: 0.0 },
        );

        // center the transform at the mouse
        let camera = shift(mouse.x, mouse.y) * main_transform * shift(-mouse.x, -mouse.y);
        let relative_cam = FractalCam {
            camera,
            min_depth: 0.0,
            min_bg_depth,
            mouse_depth,
            max_bg_depth: 0.0,
            max_mouse_depth: 0.0,
        };
        *self = (relative_cam * *self).clamp_depth();
    }

    pub fn scale(&self) -> f32 {
        let (scale, _, _) = self.camera.to_scale_rotation_translation();
        scale.y // scale.x.abs() == scale.y
    }

    pub fn min_depth(&self) -> usize {
        (self.scale().log2() + self.min_bg_depth) as usize
    }

    pub fn hover_depth(&self) -> usize {
        (self.scale().log2() + self.mouse_depth) as usize
    }

    pub fn max_depth(&self) -> usize {
        (self.scale().log2() + self.max_bg_depth) as usize
    }

    pub fn clamp_depth(self) -> Self {
        let scale = self.scale();
        let min = self.min_depth;
        let max = self.max_mouse_depth;
        let mouse_depth = self.mouse_depth.clamp(min, max);

        let min = self.min_depth;
        let max = self.max_bg_depth;
        let min_bg_depth = self.min_bg_depth.clamp(min, max);

        Self {
            min_bg_depth,
            mouse_depth,
            ..self
        }
    }
}

impl Mul for FractalCam {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            camera: self.camera * rhs.camera,
            min_depth: self.min_depth + rhs.min_depth,
            min_bg_depth: self.min_bg_depth + rhs.min_bg_depth,
            mouse_depth: self.mouse_depth + rhs.mouse_depth,
            max_bg_depth: self.max_bg_depth + rhs.max_bg_depth,
            max_mouse_depth: self.max_mouse_depth + rhs.max_mouse_depth,
        }
    }
}
