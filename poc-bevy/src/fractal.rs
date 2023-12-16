mod assets;

use crate::{debug::Blocc, io::PlanetList};

use bevy::{prelude::*, sprite::Anchor, text::Text2dBounds, utils::HashMap};
use fractory_common::sim::logic::{
    factory::{Fractory, FractoryMeta},
    planet::{Biome, BiomeId, Planet, PlanetCache, PlanetId},
    presets::{XYYY, XYYY_LANDING_ZONE},
};

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, sbinalla);
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
            Transform::from_scale(Vec2::splat(200.0).extend(1.0)),
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

#[derive(Component)]
struct Sbinalla;

fn sbinalla(time: Res<Time>, mut sbinners: Query<&mut Transform, With<Sbinalla>>) {
    let time = time.elapsed_seconds();
    for mut tf in sbinners.iter_mut() {
        tf.rotation = Quat::from_rotation_z(time);
    }
}

impl NodeEntity {
    fn spawn(
        commands: &mut Commands,
        transform: Transform,
        sprite: Handle<Image>,
        name: String,
    ) -> Entity {
        let side = 1.0;
        /// std::f32::consts::SQRT_3 is unstable so here it is
        const SQRT_3: f32 = 1.732050807568877293527446341505872367_f32;
        let out_r = SQRT_3 / 3.0 * side;
        let in_r = out_r / 2.0;

        let size = Vec2::new(side, out_r + in_r);
        let sprite = commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(size),
                    ..default()
                },
                texture: sprite,
                transform: Transform::from_xyz(0.0, in_r / 2.0, 0.0),
                ..default()
            })
            .id();

        let name = commands.spawn(text(name, 120.0, size)).id();

        let center = commands
            .spawn(
                Blocc {
                    z: 2.0,
                    w: 1.0 / 10.0,
                    h: 1.0 / 10.0,
                    color: Color::YELLOW,
                    ..default()
                }
                .bundle(),
            )
            .id();
        let top = commands
            .spawn(
                Blocc {
                    y: out_r,
                    z: 2.0,
                    w: 1.0 / 10.0,
                    h: 1.0 / 10.0,
                    color: Color::YELLOW,
                    ..default()
                }
                .bundle(),
            )
            .id();
        let bot = commands
            .spawn(
                Blocc {
                    y: -in_r,
                    z: 2.0,
                    w: 1.0 / 10.0,
                    h: 1.0 / 10.0,
                    color: Color::YELLOW,
                    ..default()
                }
                .bundle(),
            )
            .id();
        let lft = commands
            .spawn(
                Blocc {
                    x: -side / 2.0,
                    y: -in_r,
                    z: 2.0,
                    w: 1.0 / 10.0,
                    h: 1.0 / 10.0,
                    color: Color::YELLOW,
                    ..default()
                }
                .bundle(),
            )
            .id();

        let rgt = commands
            .spawn(
                Blocc {
                    x: side / 2.0,
                    y: -in_r,
                    z: 2.0,
                    w: 1.0 / 10.0,
                    h: 1.0 / 10.0,
                    color: Color::YELLOW,
                    ..default()
                }
                .bundle(),
            )
            .id();

        commands
            .spawn((
                Self { sprite, name },
                Sbinalla,
                SpatialBundle {
                    transform,
                    ..default()
                },
            ))
            .add_child(sprite)
            .add_child(name)
            .add_child(center)
            .add_child(top)
            .add_child(bot)
            .add_child(lft)
            .add_child(rgt)
            .id()
    }
}
