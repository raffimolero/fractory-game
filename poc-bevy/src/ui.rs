//! Utility stuff for ui related stuff.

pub mod prelude {
    pub use super::{elements::*, hover::prelude::*, state::prelude::*, Despawn};
}

// pub mod despawn;
pub mod elements;
pub mod hover;
pub mod state;

use crate::prelude::*;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugins((hover::Plug, state::Plug))
            .add_systems(PostUpdate, pre_despawn.in_set(PostUpdateSet::PreDespawn));
    }
}

#[derive(Component)]
pub struct Despawn;

fn pre_despawn(
    mut commands: Commands,
    mut despawning: Query<
        (Entity, &mut AnimationProgress, Option<&AnimationControl>),
        With<Despawn>,
    >,
) {
    // todo instead: set animationprogress to 0 and only despawn when all the puppets are gone
    // drop functionality will be implemented as a reversible event at animation progress 0.0/1.0
    // drops are not guaranteed to happen in the same tick, but animations are cool anyway
    // TODO: do NOT sort them all into a vec based on their depth in the hierarchy
    despawning.for_each_mut(|(e, mut progress, control)| {
        progress.0 = 0.0;
        if control.is_none_or(|control| control.puppets.is_empty()) {
            commands.entity(e).despawn_recursive();
        }
    });
}
