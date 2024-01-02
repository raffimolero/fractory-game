use std::f32::consts::TAU;

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    window::PrimaryWindow,
};

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.init_resource::<MousePos>()
            .add_systems(Startup, setup)
            .add_systems(Update, (update_mouse, control_cam).chain());
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), FractalCam::default(), MainCam));
}

#[derive(Resource, Clone, Copy, Default)]
pub struct MousePos {
    pub pos: Vec2,
    pub prev: Vec2,
}

impl MousePos {
    pub fn delta(&self) -> Vec2 {
        self.pos - self.prev
    }
}

fn update_mouse(window: Query<&Window, With<PrimaryWindow>>, mut mouse: ResMut<MousePos>) {
    let window = window.single();
    let Some(mut cursor) = window.cursor_position() else {
        return;
    };
    let resolution = &window.resolution;
    let size = Vec2::new(resolution.width(), resolution.height());
    cursor -= size / 2.0;
    cursor.y *= -1.0;
    mouse.prev = mouse.pos;
    mouse.pos = cursor;
}

#[derive(Component, Debug, Clone, Copy)]
pub struct MainCam;

#[derive(Component, Debug, Clone, Copy)]
pub struct FractalCam {
    pub min_depth: f32,
    pub min_bg_depth: f32,
    pub mouse_depth: f32,
    pub max_bg_depth: f32,
    pub max_mouse_depth: f32,
}

impl Default for FractalCam {
    fn default() -> Self {
        Self {
            min_depth: -1.0,
            min_bg_depth: 1.0,
            mouse_depth: 4.0,
            max_bg_depth: 4.0,
            max_mouse_depth: 6.0,
        }
    }
}

impl FractalCam {
    fn clamp_depth(&mut self) {
        let min = self.min_depth;
        let max = self.max_mouse_depth;
        self.mouse_depth = self.mouse_depth.clamp(min, max);

        let min = self.min_depth;
        let max = self.max_bg_depth;
        self.min_bg_depth = self.min_bg_depth.clamp(min, max);
    }
}

fn control_cam(
    time: Res<Time>,
    cursor: Res<MousePos>,
    mut camera: Query<(&mut Transform, &mut FractalCam), With<MainCam>>,
    mouse: Res<Input<MouseButton>>,
    mut scroll: EventReader<MouseWheel>,
    keys: Res<Input<KeyCode>>,
) {
    let (mut cam_tf, mut frac_cam) = camera.single_mut();

    // keyboard pan
    let delta = time.delta_seconds();
    let spd = 1000.0;
    let mut mov = Vec2::ZERO;
    if keys.pressed(KeyCode::W) {
        mov.y += 1.0;
    }
    if keys.pressed(KeyCode::S) {
        mov.y -= 1.0;
    }
    if keys.pressed(KeyCode::D) {
        mov.x += 1.0;
    }
    if keys.pressed(KeyCode::A) {
        mov.x -= 1.0;
    }
    mov = spd * delta * mov.normalize_or_zero();

    // mouse pan
    if mouse.pressed(MouseButton::Right) {
        mov -= cursor.delta();
    }

    // rotation
    let spd = TAU / 2.0;
    let mut rot = 0.0;
    if keys.pressed(KeyCode::E) {
        rot += 1.0;
    }
    if keys.pressed(KeyCode::Q) {
        rot -= 1.0;
    }
    let rot = Quat::from_rotation_z(spd * delta * rot * cam_tf.scale.x.signum());

    // keyboard zoom
    let spd = 4_f32;
    let mut scl = 0.0;
    if keys.pressed(KeyCode::ShiftLeft) {
        scl += 1.0;
    }
    if keys.pressed(KeyCode::Space) {
        scl -= 1.0;
    }
    scl = spd.powf(delta * scl);

    // scroll zoom
    for delta in scroll.read() {
        let unit = match delta.unit {
            MouseScrollUnit::Line => 0.5,
            MouseScrollUnit::Pixel => 0.01,
        };
        scl *= spd.powf(-delta.y * unit);
    }

    // apply transforms
    if keys.just_pressed(KeyCode::F) {
        cam_tf.scale.x *= -1.0;
    }
    cam_tf.translation = *cam_tf * cursor.pos.extend(0.0);

    // zoom
    if keys.pressed(KeyCode::ControlLeft) {
        // recurse mouse hovered
        frac_cam.mouse_depth += scl;
    } else if keys.pressed(KeyCode::AltLeft) {
        // recurse background
        frac_cam.min_bg_depth += scl;
    } else {
        // zoom camera
        frac_cam.mouse_depth -= scl;
        cam_tf.scale *= Vec2::splat(scl).extend(1.0);
    }
    frac_cam.clamp_depth();

    cam_tf.rotation *= rot;
    cam_tf.translation = *cam_tf * (mov - cursor.pos).extend(0.0);
}
