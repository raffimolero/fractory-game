use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};

use crate::debug::{control, WasdControl};

#[derive(Component, Debug, Clone, Copy)]
pub struct MainCam;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                lock_to_mouse.before(control),
                unlock_from_mouse.after(control),
            ),
        );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCam, WasdControl));
}

fn cursor_to_px(mut cursor: Vec2, resolution: &WindowResolution) -> Vec2 {
    let size = Vec2::new(resolution.width(), resolution.height());
    cursor -= size / 2.0;
    cursor.y *= -1.0;
    cursor
}

fn window_cursor_px_centered(window: &Window) -> Option<Vec2> {
    let cursor = window.cursor_position()?;
    Some(cursor_to_px(cursor, &window.resolution))
}

fn lock_to_mouse(
    window: Query<&Window, With<PrimaryWindow>>,
    mut camera: Query<(&mut Transform, &Camera), With<MainCam>>,
) {
    let Some(cursor) = window_cursor_px_centered(window.single()) else {
        return;
    };
    let (mut cam_tf, cam) = camera.single_mut();
    let tf = cam_tf.rotation;
    cam_tf.translation += tf * cursor.extend(0.0);
}

fn unlock_from_mouse(
    window: Query<&Window, With<PrimaryWindow>>,
    mut camera: Query<(&mut Transform, &Camera), With<MainCam>>,
) {
    let Some(cursor) = window_cursor_px_centered(window.single()) else {
        return;
    };
    let (mut cam_tf, cam) = camera.single_mut();
    let tf = cam_tf.rotation;
    cam_tf.translation -= tf * cursor.extend(0.0);
}
