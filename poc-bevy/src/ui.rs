//! Utility stuff for ui related stuff.

pub mod prelude {
    pub use super::{elements::*, hover::*, state::*, *};
}

pub mod elements;
pub mod hover;
pub mod state;

use bevy::prelude::*;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugins((hover::Plug, state::Plug))
            .add_systems(PostUpdate, (despawn, apply_deferred).chain());
    }
}

#[derive(Component)]
pub struct Despawn;

fn despawn(mut commands: Commands, despawning: Query<Entity, With<Despawn>>) {
    despawning.for_each(|e| {
        commands.entity(e).despawn_recursive();
    });
}
