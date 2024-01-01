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
    mut fragments: Query<(&IsHovered, &mut AnimationControl), With<FragmentElement>>,
) {
    let delta = time.delta_seconds();
    let rate = delta * 8.0;
    fragments.for_each_mut(|(is_hovered, mut control)| {
        control.playback_speed += rate * if is_hovered.0 { 1.0 } else { -1.0 };
        control.playback_speed = control.playback_speed.clamp(-1.0, 1.0);
    });
}

#[derive(Component)]
pub struct FractoryElement {
    meta: FractoryMeta,
    // children: HashMap<TilePos, Entity>,
}

impl FractoryElement {
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

        let fractory_elem = commands
            .spawn((
                Self { meta },
                SpatialBundle {
                    transform: Transform::from_scale(Vec2::splat(500.0).extend(1.0)),
                    ..default()
                },
            ))
            .id();
        let root_fragment = FragmentElement::spawn_unloaded(commands, fractory_elem, TilePos::UNIT);
        commands
            .entity(root_fragment)
            .insert(IsHovered(false))
            .set_parent(fractory_elem);
        fractory_elem
    }
}

#[derive(Component)]
struct Unloaded;

fn load_fragments(
    mut commands: Commands,
    fractories: Query<&FractoryElement>,
    planet_cache: Res<PlanetCache>,
    unloaded: Query<(Entity, &FragmentElement), With<Unloaded>>,
) {
    unloaded.for_each(|(entity, fragment)| {
        let FragmentInfo { tile, name, sprite } =
            FragmentElement::load(*fragment, &fractories, &planet_cache);
        FragmentElement::hydrate(&mut commands, entity, *fragment, tile, name, sprite);
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

#[derive(Component, Clone, Copy)]
pub struct FragmentElement {
    pub fractory_elem: Entity,
    pub pos: TilePos,
}

struct FragmentInfo {
    tile: Tile,
    name: String,
    sprite: Handle<Image>,
}

impl FragmentElement {
    /// spawns an unloaded fragment entity.
    fn spawn_unloaded(commands: &mut Commands, fractory_elem: Entity, pos: TilePos) -> Entity {
        commands.spawn((Self { fractory_elem, pos }, Unloaded)).id()
    }

    /// loads data needed to spawn a fragment entity.
    fn load(
        data: Self,
        fractories: &Query<&FractoryElement>,
        planet_cache: &PlanetCache,
    ) -> FragmentInfo {
        let Ok(fractory) = fractories.get(data.fractory_elem) else {
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

        FragmentInfo { tile, name, sprite }
    }

    fn hydrate(
        commands: &mut Commands,
        fragment: Entity,
        data: Self,
        tile: Tile,
        name: String,
        sprite: Handle<Image>,
    ) {
        let face = Self::spawn_face(commands, fragment, tile, name, sprite);
        Self::hydrate_base(commands, fragment, face, data);
    }

    /// takes an unloaded fragment's base entity and attaches the necessary pieces to it
    fn hydrate_base(commands: &mut Commands, fragment: Entity, face: Entity, data: Self) {
        commands
            .entity(fragment)
            .add_child(face)
            .insert((
                // attach fragment data
                Self {
                    fractory_elem: data.fractory_elem,
                    pos: data.pos,
                },
                // attach hitboxes, *without* enabling hover until the animations let us
                // IsHovered(false),
                Hitbox {
                    kind: HitboxKind::Tri { r: 1.0 },
                    cursor: None,
                },
                SpatialBundle::default(),
                // attach animations
                AutoPause,
                AnimationControlBundle::from_events(
                    0.25,
                    [
                        (
                            0.0,
                            FragmentElement::spawn_puppet_fragments(
                                data.fractory_elem,
                                data.pos,
                                fragment,
                            ),
                        ),
                        (0.125, FragmentElement::add_puppet_hitboxes()),
                    ],
                ),
            ))
            .remove::<Unloaded>();
    }

    fn spawn_face(
        commands: &mut Commands,
        base: Entity,
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
                Self::spawn_tringle(children, base, size, sprite);
                Self::spawn_name(children, base, size, name);
            })
            .id()
    }

    fn spawn_tringle(children: &mut ChildBuilder, base: Entity, size: Vec2, sprite: Handle<Image>) {
        children.spawn((
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
            AnimationPuppetBundle::track(base),
            ComponentAnimator::boxed(|tf: &mut Transform, ratio: f32| {
                let ratio = ratio * ratio;
                let scale = 1.0 - ratio;
                tf.scale = Vec2::splat(scale).extend(1.0);
                tf.rotation = Quat::from_rotation_z(-TAU * ratio);
            }),
        ));
    }

    fn spawn_name(children: &mut ChildBuilder, base: Entity, size: Vec2, name: String) {
        let text = text(name, 120.0, size);
        let base_scale = text.transform.scale;
        children.spawn((
            text,
            AnimationPuppetBundle::track(base),
            ComponentAnimator::boxed(move |tf: &mut Transform, ratio: f32| {
                let ratio = ratio * ratio;
                tf.scale = base_scale * Vec2::splat(1.0 - ratio).extend(1.0);
            }),
        ));
    }

    fn spawn_puppet_fragments(
        fractory_elem: Entity,
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
                    let child = Self::spawn_unloaded(commands, fractory_elem, pos + st);
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
}
