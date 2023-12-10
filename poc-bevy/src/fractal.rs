mod assets;

use crate::{debug::Blocc, io::PlanetList};

use bevy::{prelude::*, sprite::Anchor, text::Text2dBounds, utils::HashMap};
use fractory_common::sim::logic::{
    factory::{Fractory, FractoryMeta},
    planet::{Biome, BiomeId, Planet, PlanetCache, PlanetId},
    presets::{new_xyyy_fractory_meta, new_xyyy_planet, XYYY, XYYY_LANDING_ZONE},
};

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        // .add_systems(Update, load_folder.run_if(folder_is_loaded));
    }
}

fn setup(mut commands: Commands, mut planets: ResMut<PlanetList>) {
    FractoryEntity::spawn(
        &mut commands,
        &mut planets,
        XYYY.into(),
        XYYY_LANDING_ZONE.into(),
    );
}
// TODO: import fractory
// render a tringle
// expand it on mouse enter, collapse it on mouse exit
#[derive(Component)]
pub struct FractoryEntity {
    meta: FractoryMeta,
}

impl FractoryEntity {
    fn spawn(
        commands: &mut Commands,
        planets: &mut PlanetList,
        planet: PlanetId,
        biome: BiomeId,
    ) -> Entity {
        let meta = planets.new_fractory(planet, biome);
        let fractory = NodeEntity::spawn(commands, Color::ORANGE, "X".into());
        commands
            .entity(fractory)
            .insert(FractoryEntity { meta })
            .insert(Transform::from_scale(Vec2::splat(60.0).extend(1.0)))
            .id()
    }
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
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            rotation: default(),
            scale: Vec2::splat(1.0 / font_size).extend(1.0),
        },
        ..default()
    }
}

impl NodeEntity {
    fn spawn(commands: &mut Commands, color: Color, name: String) -> Entity {
        let size = Vec2::new(1.0, 1.0);
        let sprite = commands
            .spawn(
                Blocc {
                    w: size.x,
                    h: size.y,
                    color,
                    ..default()
                }
                .bundle(),
            )
            .id();
        let name = commands.spawn(text(name, 120.0, size)).id();
        commands
            .spawn((Self { sprite, name }, TransformBundle::default()))
            .add_child(sprite)
            .add_child(name)
            .id()
    }
}
