use std::f32::consts::TAU;

use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};

#[derive(Component, Debug, Clone, Copy)]
pub struct MainCam;

#[derive(Resource, Clone, Copy, Default)]
struct LastMousePos(Vec2);

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.init_resource::<LastMousePos>()
            .add_systems(Startup, setup)
            .add_systems(Update, (update_mouse, control).chain());
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCam));
}

fn update_mouse(
    window: Query<&Window, With<PrimaryWindow>>,
    mut last_mouse_pos: ResMut<LastMousePos>,
) {
    let window = window.single();
    let Some(mut cursor) = window.cursor_position() else {
        return;
    };
    let resolution = &window.resolution;
    let size = Vec2::new(resolution.width(), resolution.height());
    cursor -= size / 2.0;
    cursor.y *= -1.0;
    last_mouse_pos.0 = cursor;
}

fn control(
    time: Res<Time>,
    cursor: Res<LastMousePos>,
    mut camera: Query<&mut Transform, With<MainCam>>,
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
    let mov = mov.normalize_or_zero() * spd * delta;

    let spd = TAU / 2.0;
    let mut rot = 0.0;
    if keys.pressed(KeyCode::E) {
        rot += 1.0;
    }
    if keys.pressed(KeyCode::Q) {
        rot -= 1.0;
    }
    let rot = Quat::from_rotation_z(rot * spd * delta);

    let spd = 4_f32;
    let mut scl = 0.0;
    if keys.pressed(KeyCode::ShiftLeft) {
        scl += 1.0;
    }
    if keys.pressed(KeyCode::Space) {
        scl -= 1.0;
    }
    let scl = spd.powf(delta * scl);

    cam_tf.translation = *cam_tf * cursor.0.extend(0.0);
    cam_tf.scale *= scl;
    cam_tf.rotation *= rot;
    cam_tf.translation = *cam_tf * (mov - cursor.0).extend(0.0);
}
