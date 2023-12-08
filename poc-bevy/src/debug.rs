use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct WasdControl;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(Update, control);
    }
}

pub struct Blocc {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    pub h: f32,
    pub color: Color,
}

impl Blocc {
    pub fn bundle(self) -> SpriteBundle {
        let Self {
            x,
            y,
            z,
            w,
            h,
            color,
        } = self;

        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(w, h)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, z),
            ..default()
        }
    }
}

impl Default for Blocc {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 50.0,
            h: 50.0,
            color: Color::ALICE_BLUE,
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Blocc::default().bundle(), WasdControl));
}

fn control(mut controllables: Query<&mut Transform, With<WasdControl>>, keys: Res<Input<KeyCode>>) {
    let mut mov = Vec2::ZERO;
    if keys.pressed(KeyCode::W) {
        mov.y += 1.0;
    }
    if keys.pressed(KeyCode::S) {
        mov.y -= 1.0;
    }
    if keys.pressed(KeyCode::D) {
        mov.x += 1.0;
    }
    if keys.pressed(KeyCode::A) {
        mov.x -= 1.0;
    }
    let mov = mov.normalize_or_zero().extend(0.0);

    controllables.for_each_mut(|mut tf| {
        tf.translation += mov;
    });
}
