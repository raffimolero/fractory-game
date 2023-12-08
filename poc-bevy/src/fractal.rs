mod assets;

use std::{
    ffi::{OsStr, OsString},
    path::PathBuf,
};

use bevy::{asset::LoadedFolder, prelude::*, sprite::Anchor, text::Text2dBounds, utils::HashMap};
use fractory_common::sim::logic::{
    factory::{Fractory, FractoryMeta},
    planet::{Biome, Planet, PlanetCache},
    presets::{new_xyyy_fractory_meta, new_xyyy_planet},
};

use crate::debug::Blocc;

#[derive(Resource, Default)]
pub struct Planets(PlanetCache);

// TODO: import fractory
// render a tringle
// expand it on mouse enter, collapse it on mouse exit
#[derive(Component)]
pub struct FractoryEntity {
    meta: FractoryMeta,
}

#[derive(Component)]
pub struct NodeEntity {
    sprite: Entity,
    name: Entity,
    // data idk
    // tilepos?
}

fn text(value: String, font_size: f32, bounds: Vec2) -> Text2dBundle {
    Text2dBundle {
        text: Text {
            sections: vec![TextSection {
                value,
                style: TextStyle {
                    font: default(), // TODO: load a copyright free font
                    font_size,
                    color: Color::WHITE,
                },
            }],
            alignment: TextAlignment::Center,
            linebreak_behavior: bevy::text::BreakLineOn::WordBoundary,
        },
        text_anchor: Anchor::Center,
        text_2d_bounds: Text2dBounds { size: bounds },
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        ..default()
    }
}

impl NodeEntity {
    fn spawn(commands: &mut Commands, color: Color, name: String) -> Entity {
        let size = Vec2::new(60.0, 60.0);
        let sprite = commands
            .spawn(
                Blocc {
                    w: 60.0,
                    h: 60.0,
                    color,
                    ..default()
                }
                .bundle(),
            )
            .id();
        let name = commands.spawn(text(name, 30.0, size)).id();
        commands.spawn(Self { sprite, name }).id()
    }
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.init_resource::<Planets>().add_systems(Startup, setup);
        // .add_systems(Update, load_folder.run_if(folder_is_loaded));
    }
}

fn setup(mut commands: Commands, mut planets: ResMut<Planets>) {
    let meta = new_xyyy_fractory_meta(&mut planets.0);
    let fractory = NodeEntity::spawn(&mut commands, Color::ORANGE, "X".into());
    commands.entity(fractory).insert(FractoryEntity { meta });
}
