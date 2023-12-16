use bevy::prelude::*;

use crate::debug::WasdControl;

#[derive(Component, Debug, Clone, Copy)]
pub struct MainCam;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCam, WasdControl));
}
