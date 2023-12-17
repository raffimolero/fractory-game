// TODO: make this not broken

use std::hash::BuildHasherDefault;

use bevy::{asset::AssetServer, prelude::*, utils::AHasher};
use fractory_common::sim::logic::{
    factory::FractoryMeta,
    planet::{Biome, BiomeId, Planet, PlanetId},
    presets::{XYYY, XYYY_LANDING_ZONE, XYYY_SPINLESS},
};

type IndexMap<K, V> = indexmap::IndexMap<K, V, BuildHasherDefault<AHasher>>;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlanetList>()
            .add_systems(Startup, setup);
        // .add_systems(Update, load_folder.run_if(folder_is_loaded));
    }
}

// TODO: load assets
fn setup(mut commands: Commands, mut planets: ResMut<PlanetList>, mut assets: ResMut<AssetServer>) {
    // planets.add_planet(planet, new_xyyy_planet());
    // planets.add_planet(PlanetId::from(XYYY));
}

pub struct PlanetAssets {
    pub icon: Handle<Image>,
    pub fragment_icons: Vec<Handle<Image>>,
}

impl PlanetAssets {
    fn new_xyyy(asset_server: &mut AssetServer) -> Self {
        Self {
            icon: asset_server.load("content/planets/xyyy/sprites/icon.png"),
            fragment_icons: vec![asset_server.load("content/planets/xyyy/sprites/tringle.png")],
        }
    }
}

pub struct BiomeAssets {
    pub icon: Handle<Image>,
}

impl BiomeAssets {
    fn new_xyyy_landing_zone(asset_server: &mut AssetServer) -> Self {
        Self {
            icon: asset_server.load("content/planets/xyyy/sprites/landing_zone.png"),
        }
    }

    fn new_xyyy_spinless(asset_server: &mut AssetServer) -> Self {
        Self {
            icon: asset_server.load("content/planets/xyyy/sprites/spinless.png"),
        }
    }
}

#[derive(Resource, Default)]
pub struct PlanetList {
    pub planets: IndexMap<PlanetId, (Planet, PlanetAssets)>,
    pub biomes: IndexMap<(PlanetId, BiomeId), (Biome, BiomeAssets)>,
}

impl PlanetList {
    // TODO: loading
    pub fn get_or_load_planet(
        &mut self,
        asset_server: &mut AssetServer,
        planet: PlanetId,
    ) -> &(Planet, PlanetAssets) {
        Self::_get_or_load_planet(&mut self.planets, asset_server, planet)
    }
    pub fn _get_or_load_planet<'a>(
        planets: &'a mut IndexMap<PlanetId, (Planet, PlanetAssets)>,
        asset_server: &mut AssetServer,
        planet: PlanetId,
    ) -> &'a (Planet, PlanetAssets) {
        planets.entry(planet.clone()).or_insert_with(|| {
            if planet == PlanetId::from(XYYY) {
                (Planet::new_xyyy(), PlanetAssets::new_xyyy(asset_server))
            } else {
                todo!("actually load planet")
            }
        })
    }

    // TODO: loading
    pub fn get_or_load_biome(
        &mut self,
        asset_server: &mut AssetServer,
        planet: PlanetId,
        biome: BiomeId,
    ) -> &(Biome, BiomeAssets) {
        Self::_get_or_load_biome(&mut self.biomes, asset_server, planet, biome)
    }
    pub fn _get_or_load_biome<'a>(
        biomes: &'a mut IndexMap<(PlanetId, BiomeId), (Biome, BiomeAssets)>,
        asset_server: &mut AssetServer,
        planet: PlanetId,
        biome: BiomeId,
    ) -> &'a (Biome, BiomeAssets) {
        biomes
            .entry((planet.clone(), biome.clone()))
            .or_insert_with(|| match (planet.0.as_ref(), biome.0.as_ref()) {
                (XYYY, XYYY_LANDING_ZONE) => (
                    Biome::new_xyyy_landing_zone(),
                    BiomeAssets::new_xyyy_landing_zone(asset_server),
                ),
                (XYYY, XYYY_SPINLESS) => (
                    Biome::new_xyyy_spinless(),
                    BiomeAssets::new_xyyy_spinless(asset_server),
                ),
                _ => todo!("actually load biome"),
            })
    }

    pub fn new_fractory(
        &mut self,
        asset_server: &mut AssetServer,
        planet: PlanetId,
        biome: BiomeId,
    ) -> FractoryMeta {
        let (planet_data, planet_assets) =
            Self::_get_or_load_planet(&mut self.planets, asset_server, planet.clone());
        let (biome_data, biome_assets) = Self::_get_or_load_biome(
            &mut self.biomes,
            asset_server,
            planet.clone(),
            biome.clone(),
        );
        FractoryMeta::new(planet, biome, planet_data, biome_data)
    }
}

/*
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
*/