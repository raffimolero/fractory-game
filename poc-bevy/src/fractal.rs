mod step;
mod tile;

use self::tile::FractoryElement;
use crate::prelude::{presets::*, *};

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugins(tile::Plug)
            .add_systems(Startup, setup.in_set(StartupSet::Layout));
    }
}

fn setup(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut planets: ResMut<PlanetCache>,
) {
    FractoryElement::spawn(
        &mut commands,
        &mut asset_server,
        &mut planets,
        // &frag_animations,
        XYYY.into(),
        XYYY_LANDING_ZONE.into(),
    );
}
