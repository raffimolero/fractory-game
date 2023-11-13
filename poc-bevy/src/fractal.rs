mod assets;

use bevy::{asset::LoadedFolder, prelude::*, utils::HashMap};
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
        app.init_resource::<ContentFolder>()
            .add_systems(Startup, setup)
            .add_systems(Update, load_folder.run_if(folder_is_loaded));
    }
}

#[derive(Resource)]
struct Biomes {
    biomes: HashMap<String, Biome>,
}

#[derive(Resource, Default)]
struct ContentFolder {
    folder: Handle<LoadedFolder>,
    folder2: Handle<LoadedFolder>,
}

fn setup(mut commands: Commands, server: Res<AssetServer>, mut content: ResMut<ContentFolder>) {
    // TODO: figure out how to download and load biomes in bevy
    content.folder = server.load_folder("content");
    content.folder2 = server.load_folder("content/planets");
    println!("Loading...");
}

fn folder_is_loaded(
    mut ev_asset_folder: EventReader<AssetEvent<LoadedFolder>>,
    content: Res<ContentFolder>,
) -> bool {
    ev_asset_folder.read().into_iter().count() > 0
}

fn load_folder(content: Res<ContentFolder>, assets: Res<Assets<LoadedFolder>>) {
    let Some(folder) = assets
        .get(&content.folder)
        .or_else(|| assets.get(&content.folder2))
    else {
        unreachable!();
    };
    println!("folder loaded");
    dbg!(&folder.handles); // empty?
    for sub in &folder.handles {
        dbg!(&sub);
        dbg!(sub.id());
        dbg!(sub.type_id());
        dbg!(sub.path());
        dbg!();
    }
}
