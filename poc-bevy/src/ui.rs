//! Utility stuff for ui related stuff.

pub mod prelude {
    pub use super::{elements::*, hover::prelude::*, state::prelude::*};
}

pub mod despawn;
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

#[derive(Event)]
pub struct Despawn(pub Entity);

fn despawn(mut commands: Commands, objects: Query<(Entity, Parent, Children)>) {
    let mut xd;
    // TODO: sort them all into a vec based on their depth in the hierarchy
    despawning.for_each(|e| {
        commands.entity(e).despawn_recursive();
    });
}
