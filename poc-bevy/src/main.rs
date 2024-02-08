mod cam;
pub mod debug;
mod fractal;
mod io;
mod ui;

pub mod prelude {
    pub use super::{
        cam::{FractalCam, MainCam},
        debug::Blocc,
        io::PlanetCache,
        ui::prelude::*,
    };
    pub use std::f32::consts::TAU;

    pub use bevy::{
        input::mouse::{MouseScrollUnit, MouseWheel},
        prelude::*,
        sprite::Anchor,
        utils::HashMap,
        window::PrimaryWindow,
    };
    pub use fractory_common::sim::logic::{
        factory::FractoryMeta,
        fractal::TileFill,
        orientation::Orient,
        path::TilePos,
        planet::{Biome, BiomeId, Planet, PlanetId},
        presets,
        tile::{SubTile, Tile},
    };
}
use prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(io::Plug)
        .add_plugins((cam::Plug, debug::Plug, fractal::Plug, ui::Plug))
        .add_systems(Update, bevy::window::close_on_esc)
        .run()
}
