// TODO: make this not broken

struct BiomeWrapper {
    /// the biome data
    biome: Biome,

    // TODO: support animations
    /// sprites for each fractal leaf
    sprites: Vec<Handle<Image>>,

    /// the icon/thumbnail shown for this biome
    icon: Handle<Image>,
}

#[derive(Resource, Default)]
struct BiomeCache {
    biomes: HashMap<BiomeId, BiomeWrapper>,
}

#[derive(Resource, Default)]
struct ContentFolder {
    folder: Handle<LoadedFolder>,
}

fn setup(mut commands: Commands, server: Res<AssetServer>, mut content: ResMut<ContentFolder>) {
    // TODO: figure out how to download and load biomes in bevy
    content.folder = server.load_folder("content");
    println!("Loading...");
}

fn folder_is_loaded(
    mut ev_asset_folder: EventReader<AssetEvent<LoadedFolder>>,
    content: Res<ContentFolder>,
) -> bool {
    ev_asset_folder.read().into_iter().count() > 0
}

fn load_folder(
    content: Res<ContentFolder>,
    assets: Res<Assets<LoadedFolder>>,
    mut biomes: ResMut<BiomeCache>,
    images: Res<Assets<Image>>,
) {
    let folder = assets.get(&content.folder).unwrap();
    println!("folder loaded");

    let mut filtered_handles = folder
        .handles
        .iter()
        .filter_map(|handle| {
            let path = handle.path().expect("this path better exist.").path();
            if !path.starts_with("content/planets/") {
                // Not in content folder. Could be game assets.
                return None;
            }
            let Some(parent) = path.parent() else {
                println!("Parent was None. Could be user error.");
                return None;
            };
            let biome_name: &OsStr = parent.file_name().expect("this path better exist.");
            let biome_id = BiomeId::from(biome_name.to_string_lossy().into_owned());
            Some((handle, path, biome_id))
        })
        .collect::<Vec<_>>();

    // register all biome names
    filtered_handles.retain(|(_handle, path, biome)| {
        if path.file_name().expect("this path better exist.") == "icon.png" {
            if biomes.biomes.insert(biome.to_owned(), todo!()).is_some() {
                println!("WARNING: Duplicate biomes");
            }
            false
        } else {
            // Not the icon. Could be other assets.
            true
        }
    });

    // // load all biome assets
    // for &(_handle, path, biome) in &filtered_handles {
    //     if biomes.biomes.get(name.to_owned(), None).is_some() {
    //         println!("WARNING: Duplicate biomes");
    //     }
    // }
}
