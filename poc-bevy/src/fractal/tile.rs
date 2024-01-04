use crate::{
    cam::{FractalCam, MainCam},
    io::PlanetCache,
    ui::prelude::*,
};
use std::f32::consts::TAU;

use bevy::{prelude::*, sprite::Anchor};
use fractory_common::sim::logic::{
    factory::FractoryMeta,
    fractal::TileFill,
    orientation::Orient,
    path::TilePos,
    planet::{BiomeId, PlanetId},
    presets::*,
    tile::{SubTile, Tile},
};

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (load_fragments, check_fragment_expansion));
    }
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

        init_xyyy_fractory(&mut meta.fractory, Config::TestGrowFarm);

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

#[derive(Component, Clone, Copy)]
struct UnloadedFragment {
    fractory_elem: Entity,
    pos: TilePos,
}

fn load_fragments(
    mut commands: Commands,
    fractories: Query<&FractoryElement>,
    planet_cache: Res<PlanetCache>,
    unloaded: Query<(Entity, &UnloadedFragment)>,
) {
    unloaded.for_each(|(entity, fragment)| {
        let info = FragmentInfo::load(*fragment, &fractories, &planet_cache);
        FragmentElement::hydrate(&mut commands, entity, *fragment, info);
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

struct FragmentInfo {
    tile: Tile,
    fill: TileFill,
    name: String,
    face_sprite: Handle<Image>,
    slot_sprite: Handle<Image>,
}

impl FragmentInfo {
    /// loads data needed to spawn a fragment entity.
    fn load(
        data: UnloadedFragment,
        fractories: &Query<&FractoryElement>,
        planet_cache: &PlanetCache,
    ) -> FragmentInfo {
        let Ok(fractory) = fractories.get(data.fractory_elem) else {
            panic!(
                "attempted to access nonexistent fractory entity.\n\
                fractory root should've despawned before children."
            );
        };

        let fractal = &fractory.meta.fractory.fractal;
        let tile = fractal.get(data.pos);
        let fill = fractal.get_info(tile.id).fill;

        let (planet_data, planet_assets) = planet_cache
            .planets
            .get(&fractory.meta.planet)
            .expect("Planets should be loaded by now.");

        let name = planet_data
            .fragments()
            .names()
            .get(tile.id)
            .cloned()
            .unwrap_or(format!("Tile at {}", data.pos));

        let face_sprite = planet_assets.get_fragment_icon(tile.id);
        let slot_sprite = planet_assets.get_fragment_icon(0);

        FragmentInfo {
            tile,
            fill,
            name,
            face_sprite,
            slot_sprite,
        }
    }
}

fn check_fragment_expansion(
    camera: Query<(&GlobalTransform, &FractalCam), With<MainCam>>,
    mut fragments: Query<(
        &FragmentElement,
        &IsHovered,
        &GlobalTransform,
        &ViewVisibility,
        &mut AnimationDestination,
    )>,
) {
    let (cam_gtf, frac_cam) = camera.single();
    let cam_scale = cam_gtf.to_scale_rotation_translation().0.y;

    fragments.for_each_mut(|(fragment, is_hovered, gtf, visibility, mut destination)| {
        let should_expand = visibility.get() && {
            let frag_scale = gtf.to_scale_rotation_translation().0.y;
            let relative_depth = (cam_scale / frag_scale).log2() + 10.0;

            let threshold = if is_hovered.0 {
                frac_cam.mouse_depth
            } else if fragment.fill.is_leaf() {
                frac_cam.min_bg_depth
            } else {
                frac_cam.max_bg_depth
            };

            relative_depth < threshold
        };

        *destination = if should_expand {
            AnimationDestination::End
        } else {
            AnimationDestination::Start
        }
    });
}

#[derive(Component, Clone, Copy)]
pub struct FragmentElement {
    pub fractory_elem: Entity,
    pub pos: TilePos,
    pub fill: TileFill,
}

impl FragmentElement {
    /// spawns an unloaded fragment entity.
    fn spawn_unloaded(commands: &mut Commands, fractory_elem: Entity, pos: TilePos) -> Entity {
        commands.spawn(UnloadedFragment { fractory_elem, pos }).id()
    }

    fn hydrate(
        commands: &mut Commands,
        fragment: Entity,
        data: UnloadedFragment,
        info: FragmentInfo,
    ) {
        let FragmentInfo {
            tile,
            fill,
            name,
            face_sprite,
            slot_sprite,
        } = info;
        let face = Self::spawn_face(commands, fragment, tile, name, face_sprite);
        Self::hydrate_base(commands, fragment, face, data, slot_sprite, fill);
    }

    /// takes an unloaded fragment's base entity and attaches the necessary pieces to it
    fn hydrate_base(
        commands: &mut Commands,
        base: Entity,
        face: Option<Entity>,
        data: UnloadedFragment,
        slot_sprite: Handle<Image>,
        fill: TileFill,
    ) {
        let fragment_data = Self {
            fractory_elem: data.fractory_elem,
            pos: data.pos,
            fill,
        };

        let size = Vec2::new(1.0, TRI_HEIGHT);
        let slot_sprite = SpriteBundle {
            sprite: Sprite {
                custom_size: Some(size),
                anchor: Anchor::Custom(Vec2::new(0.0, -TRI_CENTER_OFF_Y)),
                ..default()
            },
            texture: slot_sprite,
            ..default()
        };

        let hitbox = (
            // IsHovered(false),
            Hitbox {
                kind: HitboxKind::Tri { r: 1.0 },
                cursor: None,
            },
        );

        let reveal_animation = ComponentAnimator::boxed(|sprite: &mut Sprite, ratio: f32| {
            let ratio = ratio * ratio;
            sprite.color = sprite.color.with_a(1.0 - ratio);
        });

        let spawn_puppet_fragments = FragmentElement::spawn_puppet_fragments(
            fragment_data.fractory_elem,
            fragment_data.pos,
            base,
        );
        let add_puppet_hitboxes = FragmentElement::add_puppet_hitboxes();
        let expand_animation = (
            AutoPause,
            AnimationControlBundle::from_events(
                0.25,
                [(0.0, spawn_puppet_fragments), (0.125, add_puppet_hitboxes)],
            ),
            AnimationDestination::Start,
        );

        commands
            .entity(base)
            .insert((
                fragment_data,
                slot_sprite,
                hitbox,
                reveal_animation,
                expand_animation,
            ))
            .remove::<UnloadedFragment>();

        if let Some(face) = face {
            commands.entity(base).add_child(face);
        }
    }

    fn spawn_face(
        commands: &mut Commands,
        base: Entity,
        tile: Tile,
        name: String,
        sprite: Handle<Image>,
    ) -> Option<Entity> {
        (tile != Tile::SPACE).then(|| {
            commands
                .spawn(SpatialBundle {
                    transform: transform_from_orient(tile.orient).with_translation(Vec3::Z),
                    ..default()
                })
                .with_children(|children| {
                    let size = Vec2::new(1.0, TRI_HEIGHT) * 0.875;
                    Self::spawn_tringle(children, base, size, sprite);
                    Self::spawn_name(children, base, size, name);
                })
                .id()
        })
    }

    fn spawn_tringle(children: &mut ChildBuilder, base: Entity, size: Vec2, sprite: Handle<Image>) {
        let hitbox = (
            Hitbox {
                kind: HitboxKind::Tri { r: 1.0 },
                cursor: Some(CursorIcon::Hand),
            },
            IsHovered(false),
        );

        let sprite = SpriteBundle {
            sprite: Sprite {
                custom_size: Some(size),
                anchor: Anchor::Custom(Vec2::new(0.0, -TRI_CENTER_OFF_Y)),
                ..default()
            },
            texture: sprite,
            ..default()
        };

        // TODO: make this snappier so we can play a juicy sound effect
        // maybe add particles too
        let reveal_animation = (
            AnimationPuppetBundle::track(base),
            ComponentAnimator::boxed(|tf: &mut Transform, ratio: f32| {
                let ratio = ratio * ratio;
                let scale = 1.0 - ratio;
                tf.scale = Vec2::splat(scale).extend(1.0);
                tf.rotation = Quat::from_rotation_z(-TAU * ratio);
            }),
        );

        children.spawn((hitbox, sprite, reveal_animation));
    }

    fn spawn_name(children: &mut ChildBuilder, base: Entity, size: Vec2, name: String) {
        let text = text(name, 120.0, size);

        let base_scale = text.transform.scale;
        let reveal_animation = (
            AnimationPuppetBundle::track(base),
            ComponentAnimator::boxed(move |tf: &mut Transform, ratio: f32| {
                let ratio = ratio * ratio;
                tf.scale = base_scale * Vec2::splat(1.0 - ratio).extend(1.0);
            }),
        );

        children.spawn((text, reveal_animation));
    }

    fn spawn_puppet_fragments(
        fractory_elem: Entity,
        pos: TilePos,
        base: Entity,
    ) -> Box<dyn ReversibleEvent> {
        // TODO: abstract spawn/despawn REvent

        REvent::boxed(
            move |commands, puppets| {
                for (subtile, xy) in SubTile::ORDER.into_iter().zip([
                    Vec2::ZERO,
                    TRI_VERTS[1] / 2.0,
                    TRI_VERTS[2] / 2.0,
                    TRI_VERTS[0] / 2.0,
                ]) {
                    let is_center = subtile == SubTile::C;
                    let rot = if is_center { TAU / 2.0 } else { 0.0 };
                    let z = if is_center { -1.0 } else { -2.0 };
                    let puppet = Self::spawn_puppet(
                        commands,
                        fractory_elem,
                        pos + subtile,
                        xy.extend(z),
                        rot,
                    );
                    commands.entity(base).add_child(puppet);
                    puppets.push(puppet);
                }
            },
            despawn_puppets,
        )
    }

    fn spawn_puppet(
        commands: &mut Commands,
        fractory_elem: Entity,
        pos: TilePos,
        translation: Vec3,
        rotation: f32,
    ) -> Entity {
        let puppet = commands
            .spawn(SpatialBundle {
                transform: Transform {
                    rotation: Quat::from_rotation_z(rotation),
                    scale: Vec3::splat(0.5),
                    translation,
                },
                ..default()
            })
            .id();
        let child = Self::spawn_unloaded(commands, fractory_elem, pos);
        commands.entity(puppet).add_child(child);
        puppet
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
