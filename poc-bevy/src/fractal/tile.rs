use crate::{io::PlanetCache, ui::prelude::*};
use std::f32::consts::TAU;

use bevy::{prelude::*, sprite::Anchor};
use fractory_common::sim::logic::{
    factory::FractoryMeta,
    orientation::{Orient, Transform as TriTf},
    path::TilePos,
    planet::{BiomeId, PlanetId},
    presets::*,
    tile::{SubTile, Tile},
};

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (load_fragments, fragment_hover));
    }
}

fn fragment_hover(
    time: Res<Time>,
    mut fragments: Query<(&IsHovered, &mut AnimationControl), With<FragmentData>>,
) {
    let delta = time.delta_seconds();
    let rate = delta * 8.0;
    fragments.for_each_mut(|(is_hovered, mut control)| {
        control.playback_speed += rate * if is_hovered.0 { 1.0 } else { -1.0 };
        control.playback_speed = control.playback_speed.clamp(-1.0, 1.0);
    });
}

#[derive(Component)]
pub struct FractoryEntity {
    meta: FractoryMeta,
    // children: HashMap<TilePos, Entity>,
}

impl FractoryEntity {
    pub fn spawn(
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
        let root_fragment = FragmentData::spawn_unloaded(commands, fractory, TilePos::UNIT);
        commands
            .entity(root_fragment)
            .insert(IsHovered(false))
            .set_parent(fractory);
        fractory
    }
}

#[derive(Component, Clone, Copy)]
struct UnloadedFragment {
    root: Entity,
    pos: TilePos,
}

fn load_fragments(
    mut commands: Commands,
    fractories: Query<&FractoryEntity>,
    planet_cache: Res<PlanetCache>,
    unloaded: Query<(Entity, &UnloadedFragment)>,
) {
    unloaded.for_each(|(fragment, data)| {
        let FragmentDataTemp { tile, name, sprite } =
            FragmentData::get_data(*data, &fractories, &planet_cache);
        FragmentData::hydrate(&mut commands, fragment, *data, tile, name, sprite);
    });
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
    pub root: Entity,
    pub pos: TilePos,
    pub tile: Tile,
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
                    let child = Self::spawn_unloaded(commands, root, pos + st);
                    commands
                        .entity(puppet)
                        .set_parent(fragment)
                        .add_child(child);
                    puppets.push(puppet);
                }
            },
            move |commands, puppets| {
                for p in puppets.drain(..) {
                    commands.entity(p).insert(Despawn);
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
                        let child = world
                            .entity(p)
                            .get::<Children>()
                            .expect("each puppet must have the actual fragment as a child")[0];
                        world.entity_mut(child).insert(IsHovered(false));
                    });
                }
            },
            |commands, puppets| {
                for p in puppets.iter().copied() {
                    commands.add(move |world: &mut World| {
                        let child = world
                            .entity(p)
                            .get::<Children>()
                            .expect("each puppet must have the actual fragment as a child")[0];
                        world.entity_mut(child).remove::<IsHovered>();
                    });
                }
            },
        )
    }

    fn get_data(
        data: UnloadedFragment,
        fractories: &Query<&FractoryEntity>,
        planet_cache: &PlanetCache,
    ) -> FragmentDataTemp {
        let Ok(fractory) = fractories.get(data.root) else {
            panic!(
                "attempted to access nonexistent fractory entity.\n\
                fractory root should've despawned before children."
            );
        };
        let tile = fractory.meta.fractory.fractal.get(data.pos);

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

        FragmentDataTemp { tile, name, sprite }
    }

    fn hydrate_base(
        commands: &mut Commands,
        fragment: Entity,
        face: Entity,
        data: UnloadedFragment,
        tile: Tile,
    ) {
        commands
            .entity(fragment)
            .add_child(face)
            .insert((
                FragmentData {
                    root: data.root,
                    pos: data.pos,
                    tile,
                },
                Hitbox {
                    kind: HitboxKind::Tri { r: 1.0 },
                    cursor: None,
                },
                SpatialBundle::default(),
                AutoPause,
                AnimationControlBundle::from_events(
                    0.25,
                    [
                        (
                            0.0,
                            FragmentData::spawn_puppet_fragments(data.root, data.pos, fragment),
                        ),
                        (0.125, FragmentData::add_puppet_hitboxes()),
                    ],
                ),
            ))
            .remove::<UnloadedFragment>();
    }

    fn hydrate_face(
        commands: &mut Commands,
        fragment: Entity,
        tile: Tile,
        name: String,
        sprite: Handle<Image>,
    ) -> Entity {
        commands
            .spawn(SpatialBundle {
                transform: transform_from_orient(tile.orient),
                ..default()
            })
            .with_children(|children| {
                let size = Vec2::new(1.0, TRI_HEIGHT) * 0.875;
                let _tringle = children.spawn((
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
                    AnimationPuppetBundle::track(fragment),
                    ComponentAnimator::boxed(|tf: &mut Transform, ratio: f32| {
                        let ratio = ratio * ratio;
                        let scale = 1.0 - ratio;
                        tf.scale = Vec2::splat(scale).extend(1.0);
                        tf.rotation = Quat::from_rotation_z(-TAU * ratio);
                    }),
                ));

                let text = text(name, 120.0, size);

                let base_scale = text.transform.scale;
                let _tag = children.spawn((
                    text,
                    AnimationPuppetBundle::track(fragment),
                    ComponentAnimator::boxed(move |tf: &mut Transform, ratio: f32| {
                        let ratio = ratio * ratio;
                        tf.scale = base_scale * Vec2::splat(1.0 - ratio).extend(1.0);
                    }),
                ));
            })
            .id()
    }

    fn hydrate(
        commands: &mut Commands,
        fragment: Entity,
        data: UnloadedFragment,
        tile: Tile,
        name: String,
        sprite: Handle<Image>,
    ) {
        let face = Self::hydrate_face(commands, fragment, tile, name, sprite);
        Self::hydrate_base(commands, fragment, face, data, tile);
    }

    fn spawn_unloaded(commands: &mut Commands, root: Entity, pos: TilePos) -> Entity {
        commands.spawn(UnloadedFragment { root, pos }).id()
    }
}

// TODO: refactor
struct FragmentDataTemp {
    tile: Tile,
    name: String,
    sprite: Handle<Image>,
}
