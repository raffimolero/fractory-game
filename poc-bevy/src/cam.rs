use std::f32::consts::TAU;

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    window::PrimaryWindow,
};

#[derive(Component, Debug, Clone, Copy)]
pub struct MainCam;

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

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.init_resource::<MousePos>()
            .add_systems(Startup, setup)
            .add_systems(Update, (update_mouse, control).chain());
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCam));
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

fn control(
    time: Res<Time>,
    cursor: Res<MousePos>,
    mut camera: Query<&mut Transform, With<MainCam>>,
    mouse: Res<Input<MouseButton>>,
    mut scroll: EventReader<MouseWheel>,
    keys: Res<Input<KeyCode>>,
) {
    let mut cam_tf = camera.single_mut();

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

    if mouse.pressed(MouseButton::Right) {
        mov -= cursor.delta();
    }

    let spd = TAU / 2.0;
    let mut rot = 0.0;
    if keys.pressed(KeyCode::E) {
        rot += 1.0;
    }
    if keys.pressed(KeyCode::Q) {
        rot -= 1.0;
    }
    let rot = Quat::from_rotation_z(spd * delta * rot * cam_tf.scale.x.signum());

    let spd = 4_f32;
    let mut scl = 0.0;
    if keys.pressed(KeyCode::ShiftLeft) {
        scl += 1.0;
    }
    if keys.pressed(KeyCode::Space) {
        scl -= 1.0;
    }
    scl = spd.powf(delta * scl);

    for delta in scroll.read() {
        let unit = match delta.unit {
            MouseScrollUnit::Line => 0.5,
            MouseScrollUnit::Pixel => 0.01,
        };
        scl *= spd.powf(-delta.y * unit);
    }

    let mut scl = Vec2::splat(scl).extend(1.0);
    if keys.just_pressed(KeyCode::F) {
        scl.x *= -1.0;
    }

    cam_tf.translation = *cam_tf * cursor.pos.extend(0.0);
    cam_tf.scale *= scl;
    cam_tf.rotation *= rot;
    cam_tf.translation = *cam_tf * (mov - cursor.pos).extend(0.0);
}
