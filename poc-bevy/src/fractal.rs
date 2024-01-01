mod step;
mod tile;

use self::tile::FractoryEntity;
use crate::io::PlanetCache;

use bevy::prelude::*;
use fractory_common::sim::logic::presets::*;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugins(tile::Plug).add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut planets: ResMut<PlanetCache>,
) {
    FractoryEntity::spawn(
        &mut commands,
        &mut asset_server,
        &mut planets,
        // &frag_animations,
        XYYY.into(),
        XYYY_LANDING_ZONE.into(),
    );
}
