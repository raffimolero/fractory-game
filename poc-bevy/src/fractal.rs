mod assets;

use std::path::Path;

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
            // .add_systems(
            //     Update,
            //     (
            //         // load_folder.run_if(folder_is_loaded)
            //     ),
            // );
        ;
    }
}

#[derive(Resource)]
struct Biomes {
    biomes: HashMap<String, Biome>,
}

#[derive(Resource, Default)]
struct ContentFolder {
    folder: Handle<LoadedFolder>,
}

fn setup(mut commands: Commands, server: Res<AssetServer>, mut content: ResMut<ContentFolder>) {
    // TODO: figure out how to download and load biomes in bevy
    let h = server.load::<Image>("icon.png");
    let icon_path = h.path().expect("i literally gave you the path").path();
    // BUG: oi, icon path is relative.
    // let root = icon_path
    //     .parent()
    //     .expect("it's an asset, it's in an asset folder");
    // let user_content = root.join("/user-content/");
    // for dir in user_content
    //     .read_dir()
    //     .expect("ERROR: Cannot find or read `assets/user-content/` as a folder.")
    // {
    //     println!("{dir:?}");
    // }

    println!("Loading...");
}

// fn folder_is_loaded(
//     mut ev_asset_folder: EventReader<AssetEvent<LoadedFolder>>,
//     content: Res<ContentFolder>,
// ) -> bool {
// }

// fn load_folder(content: Res<ContentFolder>, assets: Res<Assets<LoadedFolder>>) {
//     let Some(folder) = assets.get(&content.folder) else {
//         unreachable!();
//     };
//     println!("folder loaded");
//     dbg!(&folder.handles); // empty?
//     for sub in &folder.handles {
//         dbg!(&sub);
//         dbg!(sub.id());
//         dbg!(sub.type_id());
//         dbg!(sub.path());
//         dbg!();
//     }
// }
