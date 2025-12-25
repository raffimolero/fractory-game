use crate::prelude::*;

const ROTATION_SNAP: f32 = TAU / 6.0;

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
            mouse_depth: 3.0,
            max_bg_depth: 4.0,
            max_mouse_depth: 6.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct RelativeMotions {
    x: f32,
    y: f32,
    rot: f32,
    flip: bool,
    // the following are logarithmic
    zoom: f32,
    min_bg_depth: f32,
    mouse_depth: f32,
}

impl FractalCam {
    fn input_relative(&mut self, ctx: &Context, res: &Resources) -> RelativeMotions {
        use KeyCode::*;

        let delta = get_frame_time();
        let shift_down = is_key_down(LeftShift) || is_key_down(RightShift);
        let ctrl_down = is_key_down(LeftControl) || is_key_down(RightControl);
        let alt_down = is_key_down(LeftAlt) || is_key_down(RightAlt);

        let mut motions = RelativeMotions::default();
        let RelativeMotions {
            x,
            y,
            rot,
            flip,
            zoom,
            min_bg_depth,
            mouse_depth,
        } = &mut motions;

        // check mouse
        // let mut mouse = self.get_mouse();
        let mouse = ctx.mouse_pos().unwrap_or_default();

        let mouse_zoom = {
            // mouse = ctx.project(mouse);
            let mouse_delta = ctx.project(-mouse_delta_position());
            // when mouse scrolls up, the transform zooms in
            let (_scroll_x, scroll_y) = mouse_wheel();

            // drag controls
            if is_mouse_button_down(MouseButton::Middle) {
                *x += mouse_delta.x;
                *y += mouse_delta.y;
            }

            // zoom
            let scroll_sens = res.settings.mouse_sens.zoom_sens;
            let zoom_amount = scroll_y * scroll_sens;
            zoom_amount
        };

        // check keypresses
        if !shift_down {
            let speed = res.settings.keyboard_sens.pan_sens;
            // camera pan, `y` increases downwards
            if is_key_down(W) {
                *y += delta * speed;
            }
            if is_key_down(S) {
                *y -= delta * speed;
            }
            if is_key_down(A) {
                *x += delta * speed;
            }
            if is_key_down(D) {
                *x -= delta * speed;
            }

            // camera rotation, `rot` increases clockwise
            let sensitivity = res.settings.keyboard_sens.rotate_sens;
            if ctrl_down {
                if is_key_pressed(Q) {
                    *rot -= ROTATION_SNAP;
                }
                if is_key_pressed(E) {
                    *rot += ROTATION_SNAP;
                }
            } else {
                if is_key_down(Q) {
                    *rot -= delta * sensitivity;
                }
                if is_key_down(E) {
                    *rot += delta * sensitivity;
                }
            }

            // camera flip, `flipped` reflects horizontally, across the y-axis
            if is_key_pressed(F) {
                *flip ^= true;
            }
        }
        let keyboard_zoom = {
            // zoom
            let mut zoom = 0.0;
            if ctrl_down {
                if is_key_pressed(Z) {
                    zoom -= 1.0;
                }
                if is_key_pressed(C) {
                    zoom += 1.0;
                }
            } else {
                let zoom_sens = res.settings.keyboard_sens.zoom_sens;
                if is_key_down(Z) {
                    zoom -= delta * zoom_sens;
                }
                if is_key_down(C) {
                    zoom += delta * zoom_sens;
                }
            }
            zoom
        };

        // combine zoom from mouse and keyboard
        let zoom_amount = mouse_zoom + keyboard_zoom;
        if shift_down {
            *mouse_depth += zoom_amount;
        } else if ctrl_down {
            *min_bg_depth += zoom_amount;
        } else {
            // mouse_depth -= zoom_amount;
            *zoom += zoom_amount;
        }

        motions
    }

    fn snap_all(&mut self) -> RelativeMotions {
        let (scale, rotation, _translation) = self.camera.to_scale_rotation_translation();

        // reset flip
        let flip = scale.x < 0.0;

        // snap rotation
        let (_x, _y, z) = rotation.to_euler(EulerRot::XYZ);
        let offset = ROTATION_SNAP / 2.0;
        let rot = dist_to_snap_f32(z, ROTATION_SNAP);

        // snap zoom
        debug_assert!((scale.x.abs() - scale.y).abs() < 0.001);
        debug_assert_eq!(scale.z, 1.0);
        RelativeMotions {
            x: 0.0,
            y: 0.0,
            rot,
            flip,
            zoom: dist_to_snap_f32(scale.y.log2(), 1.0),
            min_bg_depth: dist_to_snap_f32(self.min_bg_depth, 1.0),
            mouse_depth: dist_to_snap_f32(self.mouse_depth, 1.0),
        }
    }

    fn apply_motions(&mut self, ctx: &Context, motions: RelativeMotions) {
        let RelativeMotions {
            x,
            y,
            rot,
            flip,
            zoom,
            min_bg_depth,
            mouse_depth,
        } = motions;

        let zoom = 2_f32.powf(zoom);

        let main_transform = Mat4::from_scale_rotation_translation(
            Vec3 {
                x: if flip { -zoom } else { zoom },
                y: zoom,
                z: 1.0,
            },
            Quat::from_rotation_z(rot),
            Vec3 { x, y, z: 0.0 },
        );

        let mouse = ctx.mouse_pos().unwrap_or_default();
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

    /// transforms this camera based on user input
    pub fn input(&mut self, ctx: &Context, res: &Resources) {
        use KeyCode::*;

        let shift_down = is_key_down(LeftShift) || is_key_down(RightShift);
        let ctrl_down = is_key_down(LeftControl) || is_key_down(RightControl);
        let alt_down = is_key_down(LeftAlt) || is_key_down(RightAlt);

        // mutable variables that will be applied as
        // relative transforms to the camera later

        // snap all controls
        let motions = if is_key_pressed(X) {
            self.snap_all()
        } else {
            self.input_relative(ctx, res)
        };

        self.apply_motions(ctx, motions);
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
