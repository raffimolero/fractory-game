// TODO: import and call loop from logic crate
// scratch that just implement everything here because heck you that's why

// NOTE: this will come in handy
mod sub_window;

use bevy::{input::mouse::MouseMotion, prelude::*, render::view::RenderLayers};

const TRINGLE: &'static str = "sprites/tringle.png";
const BOCCS: &'static str = "sprites/boccs.png";
const FONT: &'static str = "fonts/VarelaRound-Regular.ttf";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(load_stuff)
        .add_startup_system(setup)
        .add_system(microscope)
        .insert_resource(None::<DefaultFont>)
        .run()
}

struct DefaultFont(Handle<Font>);

/// may not be necessary actually
fn load_stuff(asset_server: Res<AssetServer>) {}

#[derive(Component)]
struct Microscope;
fn microscope(
    windows: Res<Windows>,
    mut mouse_motions: EventReader<MouseMotion>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut cam: Query<(&mut Transform, &Camera), With<Microscope>>,
) {
    // let window = windows.get_primary().unwrap();
    // // window.
    if mouse_buttons.pressed(MouseButton::Left) {

    }
    cams.for_each_mut(|(tf, cam)| {});
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let width: u32 = 300;
    let height: u32 = 200;

    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(RenderLayers::layer(0))
        .insert(Microscope);

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb_u8(255, 128, 64),
            custom_size: Some(Vec2::new(width as f32, height as f32)),
            ..default()
        },
        ..default()
    });

    let style = TextStyle {
        font: asset_server.load(FONT),
        font_size: 60.0,
        color: Color::WHITE,
    };
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_sections([
            TextSection::new("Hello, ", style.clone()),
            TextSection::new(
                "World!",
                TextStyle {
                    font_size: 90.0,
                    color: Color::rgb_u8(64, 128, 255),
                    ..style.clone()
                },
            ),
        ])
        .with_alignment(TextAlignment::CENTER),
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        ..default()
    });
}
