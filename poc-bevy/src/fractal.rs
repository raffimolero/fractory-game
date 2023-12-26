use std::{f32::consts::TAU, time::Duration};

use crate::{debug::Blocc, io::PlanetCache, ui::prelude::*};

use bevy::{prelude::*, sprite::Anchor, text::Text2dBounds};
// use bevy_tweening::{
//     lens::{TransformPositionLens, TransformRotateZLens, TransformScaleLens},
//     *,
// };
use fractory_common::sim::logic::{
    factory::FractoryMeta,
    path::TilePos,
    planet::{BiomeId, PlanetId},
    presets::*,
    tile::{Quad, SubTile, Tile},
};

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            // .init_resource::<FragmentAnimations>()
            .add_systems(Startup, setup)
            .add_systems(Update, (load, fragment_hover))
            // .add_systems(Update, load_folder.run_if(folder_is_loaded));
        ;
    }
}

fn setup(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut planets: ResMut<PlanetCache>,
    // animations: Res<Assets<AnimationClip>>,
    // frag_animations: Res<FragmentAnimations>,
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
    mut fragments: Query<
        (&IsHovered, &mut AnimationControl),
        (Changed<IsHovered>, With<FragmentData>),
    >,
) {
    fragments.for_each_mut(|(is_hovered, mut control)| {
        control.playback_speed = if is_hovered.0 { 1.0 } else { -1.0 };
    });
}

// #[derive(Resource)]
// struct FragmentAnimations {
//     names: Quad<Name>,
//     animations: Quad<Handle<AnimationClip>>,
// }

// impl FromWorld for FragmentAnimations {
//     fn from_world(world: &mut World) -> Self {
//         let names = Quad([
//             Name::new("C"),
//             Name::new("U"),
//             Name::new("R"),
//             Name::new("L"),
//         ]);

//         let Some(mut animations) = world.get_resource_mut::<Assets<AnimationClip>>() else {
//             panic!("animationclip assets not initialized yet");
//         };

//         let mut c = AnimationClip::default();
//         c.add_curve_to_path(
//             EntityPath {
//                 parts: vec![names[SubTile::C].clone()],
//             },
//             VariableCurve {
//                 keyframe_timestamps: vec![0.0, 1.0],
//                 keyframes: Keyframes::Scale(vec![Vec3::ONE, Vec2::splat(0.5).extend(1.0)]),
//             },
//         );
//         let c = animations.add(c);

//         let mut u = AnimationClip::default();
//         let u = animations.add(u);

//         let mut r = AnimationClip::default();
//         let r = animations.add(r);

//         let mut l = AnimationClip::default();
//         let l = animations.add(l);

//         Self {
//             names,
//             animations: Quad([c, u, r, l]),
//         }
//     }
// }

// fn fragment_hover(
//     time: Res<Time>,
//     mut fragments: Query<(&IsHovered, &mut Animator<Transform>), With<FragmentData>>,
// ) {
//     let delta = time.delta();
//     for (is_hovered, mut animator) in fragments.iter_mut() {
//         if is_hovered.0 {
//             animator.set_speed(1.0);
//         } else {
//             // HACK: manually reverse the animation
//             animator.set_speed(0.0);
//             let tweenable = animator.tweenable_mut();
//             let elapsed = tweenable.elapsed();
//             tweenable.set_elapsed(elapsed.saturating_sub(delta));
//         }
//     }
// }

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
            .set(TilePos::UNIT, TILES[tiles::SPINNER]);

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
        commands.entity(root_fragment).insert(IsHovered(false));
        commands.entity(fractory).add_child(root_fragment);
        fractory
    }
}

#[derive(Component)]
struct Unloaded {
    tracking: Entity,
    root: Entity,
    pos: TilePos,
}

fn load(
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
        let id = fractory.meta.fractory.fractal.get(unloaded.pos).id;

        let (planet_data, planet_assets) = planet_cache
            .planets
            .get(&fractory.meta.planet)
            .expect("Planets should be loaded by now.");
        let name = planet_data
            .fragments()
            .names()
            .get(id)
            .cloned()
            .unwrap_or(format!("<#{id}>"));

        let sprite = planet_assets.get_fragment_icon(id);

        let size = Vec2::new(1.0, TRI_HEIGHT);
        let tringle = commands
            .spawn((
                // frag_animations.names[SubTile::C].clone(),
                // Self::center_tween(),
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
                    let ratio = ratio * ratio * ratio;
                    let scale = 1.0 - ratio;
                    tf.scale = Vec2::splat(scale).extend(1.0);
                    tf.rotation = Quat::from_rotation_z(TAU * ratio);
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
                    let ratio = ratio * ratio * ratio;
                    tf.scale = base_scale * Vec2::splat(1.0 - ratio).extend(1.0);
                }),
            ))
            .id();

        commands
            .entity(entity)
            .push_children(&[tringle, tag])
            .remove::<Unloaded>();
    })
}

#[derive(Component)]
pub struct FragmentData {
    root: Entity,
    id: usize,
    pos: TilePos,
}

impl FragmentData {
    // fn center_tween() -> impl Bundle {
    //     let duration = Duration::from_millis(250);
    //     let easing = EaseFunction::CubicInOut;
    //     let shrink = Tween::new(easing, duration, TransformFractalLens);
    //     Animator::new(shrink).with_speed(0.0)
    // }

    fn spawn(commands: &mut Commands, root: Entity, pos: TilePos) -> Entity {
        // TODO: abstract spawn/despawn REvent

        let fragment = commands
            .spawn((
                Self { root, id: 0, pos },
                Hitbox {
                    kind: HitboxKind::Tri { r: 1.0 },
                    cursor: Some(CursorIcon::Hand),
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

        let spawn_despawn = {
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
                        let next = commands
                            .spawn((SpatialBundle {
                                transform: Transform {
                                    rotation: Quat::from_rotation_z(rot + -TAU),
                                    scale: Vec3::splat(0.5),
                                    translation: xy.extend(z),
                                },
                                ..default()
                            },))
                            .id();
                        let child = Self::spawn(commands, root, pos + st);
                        commands.entity(next).set_parent(fragment).add_child(child);
                        puppets.push(next);
                    }
                },
                move |commands, puppets| {
                    for p in puppets.drain(1..) {
                        commands.entity(p).despawn_recursive();
                    }
                },
            )
        };

        let activate_deactivate = REvent::boxed(
            |commands, puppets| {
                dbg!();
                for p in puppets.iter().copied() {
                    commands.entity(p).insert(IsHovered(false));
                }
            },
            |commands, puppets| {
                for p in puppets.iter().copied() {
                    commands.entity(p).remove::<IsHovered>();
                }
            },
        );

        commands.entity(fragment).add_child(face).insert((
            AutoPause,
            AnimationControlBundle::from_events(
                0.5,
                [(0.0, spawn_despawn), (0.5, activate_deactivate)],
            )
            .with_puppets([face]),
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

// struct TransformFractalLens;

// impl Lens<Transform> for TransformFractalLens {
//     fn lerp(&mut self, target: &mut Transform, ratio: f32) {
//         target.scale = Vec2::splat(1.0 - ratio / 2.0).extend(1.0);
//         target.rotation = Quat::from_rotation_z(TAU / -2.0 * ratio);
//     }
// }
