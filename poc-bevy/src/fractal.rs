mod assets;

use std::{
    ffi::{OsStr, OsString},
    path::PathBuf,
};

use bevy::{asset::LoadedFolder, prelude::*, utils::HashMap};
use fractory_common::sim::logic::{
    factory::{Fractory, FractoryMeta},
    planet::{Biome, Planet, PlanetCache},
    presets::{new_xyyy_fractory_meta, new_xyyy_planet},
};

#[derive(Resource, Default)]
pub struct Planets(PlanetCache);

// TODO: import fractory
// render a tringle
// expand it on mouse enter, collapse it on mouse exit
#[derive(Component, Debug)]
pub struct FractoryEntity {
    meta: FractoryMeta,
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.init_resource::<Planets>().add_systems(Startup, setup);
        // .add_systems(Update, load_folder.run_if(folder_is_loaded));
    }
}

fn setup(mut commands: Commands, mut planets: ResMut<Planets>) {
    let fractory = commands.spawn(FractoryEntity {
        meta: new_xyyy_fractory_meta(&mut planets.0),
    });
}
