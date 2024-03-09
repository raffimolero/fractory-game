use bevy::{asset::LoadedFolder, prelude::*};
use fractory_common::sim::logic::factory::{Biome, BiomeId, Fractory};

// TODO: import fractory
// render a tringle
// expand it on mouse enter, collapse it on mouse exit
#[derive(Component, Debug)]
pub struct Factory {
    factory: Fractory,
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup.in_set(StartupSet::Load));
    }
}

struct Planets

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    // TODO: figure out how to download and load biomes in bevy
    let xyyy: Handle<LoadedFolder> = server.load_folder("xyyy");
}
