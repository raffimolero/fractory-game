use bevy::prelude::*;
use common::sim::logic::BiomeId;

// TODO: import fractory
// render a tringle
// expand it on mouse enter, collapse it on mouse exit
#[derive(Component, Debug, Clone)]
pub struct Factory {
    factory: Fractory,
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Factory {});
}
