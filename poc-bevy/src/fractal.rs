use crate::{
    debug::{Blocc, Sbinalla},
    io::PlanetList,
    ui::{Hitbox, HitboxKind, TRI_CENTER_OFF_Y, TRI_HEIGHT, TRI_INSC_R},
};

use bevy::{prelude::*, sprite::Anchor, text::Text2dBounds};
use fractory_common::sim::logic::{
    factory::FractoryMeta,
    planet::{BiomeId, PlanetId},
    presets::{XYYY, XYYY_LANDING_ZONE},
};

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        // .add_systems(Update, load_folder.run_if(folder_is_loaded));
    }
}

fn setup(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut planets: ResMut<PlanetList>,
) {
    FractoryEntity::spawn(
        &mut commands,
        &mut asset_server,
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
        asset_server: &mut AssetServer,
        planets: &mut PlanetList,
        planet: PlanetId,
        biome: BiomeId,
    ) -> Entity {
        let (_planet_data, planet_assets) =
            planets.get_or_load_planet(asset_server, planet.clone());
        let sprite = planet_assets.fragment_icons[0].clone();
        let meta = planets.new_fractory(asset_server, planet, biome);
        let fractory = NodeEntity::spawn(
            commands,
            Transform::from_scale(Vec2::splat(500.0).extend(1.0)),
            sprite,
            "X".into(),
        );
        commands
            .entity(fractory)
            .insert(FractoryEntity { meta })
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
            scale: Vec2::splat(0.5 / font_size).extend(1.0),
        },
        ..default()
    }
}

impl NodeEntity {
    fn spawn(
        commands: &mut Commands,
        transform: Transform,
        sprite: Handle<Image>,
        name: String,
    ) -> Entity {
        let size = Vec2::new(1.0, TRI_HEIGHT);
        let sprite = commands
            .spawn((SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(size),
                    ..default()
                },
                texture: sprite,
                transform: Transform::from_xyz(0.0, TRI_CENTER_OFF_Y, 0.0),
                ..default()
            },))
            .id();
        let name = commands.spawn(text(name, 120.0, size)).id();
        commands
            .spawn((
                Self { sprite, name },
                Hitbox {
                    kind: HitboxKind::Tri { r: 1.0 },
                    cursor: Some(CursorIcon::Hand),
                },
                Sbinalla,
                SpatialBundle {
                    transform,
                    ..default()
                },
            ))
            .add_child(sprite)
            .add_child(name)
            .id()
    }
}
