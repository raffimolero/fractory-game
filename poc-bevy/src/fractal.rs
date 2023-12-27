use std::f32::consts::TAU;

use crate::{io::PlanetCache, ui::prelude::*};

use bevy::{prelude::*, sprite::Anchor, text::Text2dBounds};
// use bevy_tweening::{
//     lens::{TransformPositionLens, TransformRotateZLens, TransformScaleLens},
//     *,
// };
use fractory_common::sim::logic::{
    factory::FractoryMeta,
    orientation::{Orient, Transform as TriTf},
    path::TilePos,
    planet::{BiomeId, PlanetId},
    presets::*,
    tile::SubTile,
};

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, (load_fragments, fragment_hover))
            // .add_systems(Update, load_folder.run_if(folder_is_loaded));
        ;
    }
}

fn setup(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut planets: ResMut<PlanetCache>,
) {
    FractoryEntity::spawn(
        &mut commands,
        &mut asset_server,
        &mut planets,
        // &frag_animations,
        XYYY.into(),
        XYYY_LANDING_ZONE.into(),
    );
}

fn fragment_hover(
    time: Res<Time>,
    mut fragments: Query<
        (&IsHovered, &mut AnimationControl, &AnimationProgress),
        With<FragmentData>,
    >,
) {
    let delta = time.delta_seconds();
    let rate = delta * 8.0;
    fragments.for_each_mut(|(is_hovered, mut control, progress)| {
        control.playback_speed += rate * if is_hovered.0 { 1.0 } else { -1.0 };
        control.playback_speed = control.playback_speed.clamp(-1.0, 1.0);
    });
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
        planets: &mut PlanetCache,
        planet: PlanetId,
        biome: BiomeId,
    ) -> Entity {
        let mut meta = planets.new_fractory(asset_server, planet, biome);

        meta.fractory
            .fractal
            .set(TilePos::UNIT, TILES[tiles::SPINNER].transformed(TriTf::FR));

        let fractory = commands
            .spawn((
                Self { meta },
                SpatialBundle {
                    transform: Transform::from_scale(Vec2::splat(500.0).extend(1.0)),
                    ..default()
                },
            ))
            .id();
        let root_fragment = FragmentData::spawn(commands, fractory, TilePos::UNIT);
        commands
            .entity(root_fragment)
            .try_insert(IsHovered(false))
            .set_parent(fractory);
        fractory
    }
}

#[derive(Component)]
struct Unloaded {
    tracking: Entity,
    root: Entity,
    pos: TilePos,
}

fn load_fragments(
    mut commands: Commands,
    fractories: Query<&FractoryEntity>,
    planet_cache: Res<PlanetCache>,
    unloaded: Query<(Entity, &Unloaded)>,
) {
    unloaded.for_each(|(entity, unloaded)| {
        let Ok(fractory) = fractories.get(unloaded.root) else {
            panic!(
                "attempted to access nonexistent fractory entity.\n\
                fractory root should've despawned before children."
            );
        };
        let tile = fractory.meta.fractory.fractal.get(unloaded.pos);

        let (planet_data, planet_assets) = planet_cache
            .planets
            .get(&fractory.meta.planet)
            .expect("Planets should be loaded by now.");
        let name = planet_data
            .fragments()
            .names()
            .get(tile.id)
            .cloned()
            .unwrap_or(format!("<#{}>", tile.id));

        let sprite = planet_assets.get_fragment_icon(tile.id);

        let size = Vec2::new(1.0, TRI_HEIGHT) * 0.85;
        let tringle = commands
            .spawn((
                Hitbox {
                    kind: HitboxKind::Tri { r: 1.0 },
                    cursor: Some(CursorIcon::Hand),
                },
                IsHovered(false),
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(size),
                        anchor: Anchor::Custom(Vec2::new(0.0, -TRI_CENTER_OFF_Y)),
                        ..default()
                    },
                    texture: sprite,
                    ..default()
                },
                AnimationPuppetBundle::track(unloaded.tracking),
                ComponentAnimator::boxed(|tf: &mut Transform, ratio: f32| {
                    let ratio = ratio * ratio;
                    let scale = 1.0 - ratio;
                    tf.scale = Vec2::splat(scale).extend(1.0);
                    tf.rotation = Quat::from_rotation_z(-TAU * ratio);
                }),
            ))
            .id();

        let text = text(name, 120.0, size);

        let base_scale = text.transform.scale;
        let tag = commands
            .spawn((
                text,
                AnimationPuppetBundle::track(unloaded.tracking),
                ComponentAnimator::boxed(move |tf: &mut Transform, ratio: f32| {
                    let ratio = ratio * ratio;
                    tf.scale = base_scale * Vec2::splat(1.0 - ratio).extend(1.0);
                }),
            ))
            .id();

        commands
            .entity(entity)
            .try_insert(transform_from_orient(tile.orient))
            .push_children(&[tringle, tag])
            .remove::<Unloaded>();
    })
}

fn transform_from_orient(orient: Orient) -> Transform {
    let tf = orient.to_transform();
    let angle = tf.rotation() as u8 as f32 * TAU / 3.0;
    let scale_x = if tf.reflected() { -1.0 } else { 1.0 };
    Transform {
        translation: default(),
        rotation: Quat::from_rotation_z(angle),
        scale: Vec3::new(scale_x, 1.0, 1.0),
    }
}

#[derive(Component)]
pub struct FragmentData {
    root: Entity,
    id: usize,
    pos: TilePos,
}

impl FragmentData {
    fn spawn_puppet_fragments(
        root: Entity,
        pos: TilePos,
        fragment: Entity,
    ) -> Box<dyn ReversibleEvent> {
        // TODO: abstract spawn/despawn REvent

        REvent::boxed(
            move |commands, puppets| {
                for (st, tl) in SubTile::ORDER.into_iter().zip([
                    Vec2::ZERO,
                    TRI_VERTS[1],
                    TRI_VERTS[2],
                    TRI_VERTS[0],
                ]) {
                    let is_center = st == SubTile::C;
                    let rot = if is_center { TAU / 2.0 } else { 0.0 };
                    let z = if is_center { -1.0 } else { -2.0 };
                    let xy = tl / 2.0;
                    let puppet = commands
                        .spawn(SpatialBundle {
                            transform: Transform {
                                rotation: Quat::from_rotation_z(rot + -TAU),
                                scale: Vec3::splat(0.5),
                                translation: xy.extend(z),
                            },
                            ..default()
                        })
                        .id();
                    let child = Self::spawn(commands, root, pos + st);
                    commands
                        .entity(puppet)
                        .set_parent(fragment)
                        .add_child(child);
                    puppets.push(puppet);
                }
            },
            move |commands, puppets| {
                for p in puppets.drain(..) {
                    commands.entity(p).despawn_recursive();
                }
            },
        )
    }

    fn add_puppet_hitboxes() -> Box<dyn ReversibleEvent> {
        // TODO: abstract insert/remove REvent
        // also abstract parent/child hierarchy traversal

        REvent::boxed(
            |commands, puppets| {
                for p in puppets.iter().copied() {
                    commands.add(move |world: &mut World| {
                        let Some(e) = world.get_entity(p) else {
                            return;
                        };
                        let Some(children) = e.get::<Children>() else {
                            return;
                        };
                        world.entity_mut(children[0]).insert(IsHovered(false));
                    });
                }
            },
            |commands, puppets| {
                for p in puppets.iter().copied() {
                    commands.add(move |world: &mut World| {
                        let Some(e) = world.get_entity(p) else {
                            return;
                        };
                        let Some(children) = e.get::<Children>() else {
                            return;
                        };
                        world.entity_mut(children[0]).remove::<IsHovered>();
                    });
                }
            },
        )
    }

    fn spawn(commands: &mut Commands, root: Entity, pos: TilePos) -> Entity {
        let fragment = commands
            .spawn((
                Hitbox {
                    kind: HitboxKind::Tri { r: 1.0 },
                    cursor: None,
                },
                SpatialBundle::default(),
            ))
            .id();

        let face = commands
            .spawn((
                Unloaded {
                    tracking: fragment,
                    root,
                    pos,
                },
                SpatialBundle::default(),
            ))
            .id();

        commands.entity(fragment).add_child(face).try_insert((
            Self { root, id: 0, pos },
            AutoPause,
            AnimationControlBundle::from_events(
                0.25,
                [
                    (0.0, Self::spawn_puppet_fragments(root, pos, fragment)),
                    (0.125, Self::add_puppet_hitboxes()),
                ],
            ),
        ));
        fragment
    }
}

fn text(value: String, font_size: f32, mut bounds: Vec2) -> Text2dBundle {
    let size = 0.5 / font_size / (value.len() as f32).sqrt();
    bounds /= size * 2.0;
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
            translation: Vec3::new(0.0, 0.0, 0.5),
            rotation: default(),
            scale: Vec2::splat(size).extend(1.0),
        },
        ..default()
    }
}
