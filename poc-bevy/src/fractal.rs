use std::{f32::consts::TAU, time::Duration};

use crate::{debug::Blocc, io::PlanetList, ui::prelude::*};

use bevy::{prelude::*, sprite::Anchor, text::Text2dBounds};
// use bevy_tweening::{
//     lens::{TransformPositionLens, TransformRotateZLens, TransformScaleLens},
//     *,
// };
use fractory_common::sim::logic::{
    factory::FractoryMeta,
    planet::{BiomeId, PlanetId},
    presets::{XYYY, XYYY_LANDING_ZONE},
    tile::{Quad, SubTile},
};

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            // .init_resource::<FragmentAnimations>()
            .add_systems(Startup, setup)
            .add_systems(Update, fragment_hover)
            // .add_systems(Update, load_folder.run_if(folder_is_loaded));
        ;
    }
}

fn setup(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut planets: ResMut<PlanetList>,
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
        planets: &mut PlanetList,
        planet: PlanetId,
        biome: BiomeId,
    ) -> Entity {
        let (_planet_data, planet_assets) =
            planets.get_or_load_planet(asset_server, planet.clone());
        let sprite = planet_assets.fragment_icons[0].clone();
        let meta = planets.new_fractory(asset_server, planet, biome);
        let root_fragment = FragmentData::spawn(commands, sprite, "X".into());
        commands
            .spawn((
                Self { meta },
                SpatialBundle {
                    transform: Transform::from_scale(Vec2::splat(500.0).extend(1.0)),
                    ..default()
                },
            ))
            .add_child(root_fragment)
            .id()
    }
}

#[derive(Component)]
pub struct FragmentData {
    sprite: Entity,
    name: Entity,
    // data idk
    // tilepos?
}

impl FragmentData {
    // fn center_tween() -> impl Bundle {
    //     let duration = Duration::from_millis(250);
    //     let easing = EaseFunction::CubicInOut;
    //     let shrink = Tween::new(easing, duration, TransformFractalLens);
    //     Animator::new(shrink).with_speed(0.0)
    // }

    fn spawn(
        commands: &mut Commands,
        // frag_animations: &FragmentAnimations,
        sprite: Handle<Image>,
        name: String,
    ) -> Entity {
        let size = Vec2::new(1.0, TRI_HEIGHT);

        let sprite = commands
            .spawn((
                // frag_animations.names[SubTile::C].clone(),
                // Self::center_tween(),
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(size),
                        ..default()
                    },
                    texture: sprite,
                    transform: Transform::from_xyz(0.0, TRI_CENTER_OFF_Y, 0.0),
                    ..default()
                },
            ))
            .id();
        let name = commands.spawn(text(name, 120.0, size)).id();

        // let mut player = AnimationPlayer::default();
        // player.play(frag_animations.animations[SubTile::C].clone());

        commands
            .spawn((
                Self { sprite, name },
                Hitbox {
                    kind: HitboxKind::Tri { r: 1.0 },
                    cursor: Some(CursorIcon::Hand),
                },
                IsHovered(false),
                SpatialBundle::default(),
                AnimationBundle::from_events(
                    1.0,
                    [
                        (
                            0.0,
                            REvent::boxed(
                                |_| println!("FORWARD ZERO FIRST"),
                                |_| println!("BACKWARD ZERO FIRST"),
                            ),
                        ),
                        (
                            0.0,
                            REvent::boxed(
                                |_| println!("FORWARD ZERO LAST"),
                                |_| println!("BACKWARD ZERO LAST"),
                            ),
                        ),
                        (
                            0.5,
                            REvent::boxed(
                                |_| println!("FORWARD HALF FIRST"),
                                |_| println!("BACKWARD HALF FIRST"),
                            ),
                        ),
                        (
                            0.5,
                            REvent::boxed(
                                |_| println!("FORWARD HALF LAST"),
                                |_| println!("BACKWARD HALF LAST"),
                            ),
                        ),
                        (
                            1.0,
                            REvent::boxed(
                                |_| println!("FORWARD ONE FIRST"),
                                |_| println!("BACKWARD ONE FIRST"),
                            ),
                        ),
                        (
                            1.0,
                            REvent::boxed(
                                |_| println!("FORWARD ONE LAST"),
                                |_| println!("BACKWARD ONE LAST"),
                            ),
                        ),
                    ],
                ),
                // frag_animations.names[SubTile::C].clone(),
                // player,
            ))
            .add_child(sprite)
            .add_child(name)
            .id()
    }
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

// struct TransformFractalLens;

// impl Lens<Transform> for TransformFractalLens {
//     fn lerp(&mut self, target: &mut Transform, ratio: f32) {
//         target.scale = Vec2::splat(1.0 - ratio / 2.0).extend(1.0);
//         target.rotation = Quat::from_rotation_z(TAU / -2.0 * ratio);
//     }
// }
