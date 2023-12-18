use std::{f32::consts::TAU, time::Duration};

use crate::{debug::Blocc, io::PlanetList, ui::prelude::*};

use bevy::{prelude::*, sprite::Anchor, text::Text2dBounds};
use bevy_tweening::{
    lens::{TransformPositionLens, TransformRotateZLens, TransformScaleLens},
    *,
};
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
        let fractory = NodeEntity::spawn(commands, Transform::IDENTITY, sprite, "X".into());
        commands
            .spawn((
                FractoryEntity { meta },
                SpatialBundle {
                    transform: Transform::from_scale(Vec2::splat(500.0).extend(1.0)),
                    ..default()
                },
            ))
            .add_child(fractory)
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

struct TransformFractalLens;

impl Lens<Transform> for TransformFractalLens {
    fn lerp(&mut self, target: &mut Transform, ratio: f32) {
        target.scale = Vec2::splat(1.0 - ratio / 2.0).extend(1.0);
        target.rotation = Quat::from_rotation_z(TAU / 2.0 * ratio);
    }
}

impl NodeEntity {
    fn split_tween() -> impl Bundle {
        let duration = Duration::from_secs(1);
        let easing = EaseFunction::CubicInOut;
        let shrink = Tween::new(easing, duration, TransformFractalLens);
        (Animator::new(shrink),)
    }

    fn spawn(
        commands: &mut Commands,
        transform: Transform,
        sprite: Handle<Image>,
        name: String,
    ) -> Entity {
        let size = Vec2::new(1.0, TRI_HEIGHT);
        let sprite = commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(size),
                    ..default()
                },
                texture: sprite,
                transform: Transform::from_xyz(0.0, TRI_CENTER_OFF_Y, 0.0),
                ..default()
            })
            .id();
        let name = commands.spawn(text(name, 120.0, size)).id();

        commands
            .spawn((
                Self { sprite, name },
                Hitbox {
                    kind: HitboxKind::Tri { r: 1.0 },
                    cursor: Some(CursorIcon::Hand),
                },
                Hovered(false),
                SpatialBundle {
                    transform,
                    ..default()
                },
                Self::split_tween(),
            ))
            .add_child(sprite)
            .add_child(name)
            .id()
    }
}
