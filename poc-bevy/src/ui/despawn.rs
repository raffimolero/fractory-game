use crate::prelude::*;

// TODO: plugin, then make sure that the function only uses publicly available methods

/// Drops the given entity and all its children recursively
#[derive(Debug)]
pub struct DropRecursive {
    /// Target entity
    pub entity: Entity,
}

/// Drops the given entity's children recursively
#[derive(Debug)]
pub struct DropChildrenRecursive {
    /// Target entity
    pub entity: Entity,
}

/// Function for droping an entity and all its children
pub fn drop_with_children_recursive(world: &mut World, entity: Entity) {
    // first, make the entity's own parent forget about it
    if let Some(parent) = world.get::<Parent>(entity).map(|parent| parent.get()) {
        if let Some(mut children) = world.get_mut::<Children>(parent) {
            children.0.retain(|c| *c != entity);
        }
    }

    // then drop the entity and all of its children
    drop_with_children_recursive_inner(world, entity);
}

// Should only be called by `drop_with_children_recursive`!
fn drop_with_children_recursive_inner(world: &mut World, entity: Entity) {
    if let Some(mut children) = world.get_mut::<Children>(entity) {
        for e in std::mem::take(&mut children.0) {
            drop_with_children_recursive_inner(world, e);
        }
    }

    if !world.drop(entity) {
        debug!("Failed to drop entity {:?}", entity);
    }
}

fn drop_children_recursive(world: &mut World, entity: Entity) {
    if let Some(children) = world.entity_mut(entity).take::<Children>() {
        for e in children.0 {
            drop_with_children_recursive_inner(world, e);
        }
    }
}

impl Command for DropRecursive {
    fn apply(self, world: &mut World) {
        #[cfg(feature = "trace")]
        let _span = bevy_utils::tracing::info_span!(
            "command",
            name = "DropRecursive",
            entity = bevy_utils::tracing::field::debug(self.entity)
        )
        .entered();
        drop_with_children_recursive(world, self.entity);
    }
}

impl Command for DropChildrenRecursive {
    fn apply(self, world: &mut World) {
        #[cfg(feature = "trace")]
        let _span = bevy_utils::tracing::info_span!(
            "command",
            name = "DropChildrenRecursive",
            entity = bevy_utils::tracing::field::debug(self.entity)
        )
        .entered();
        drop_children_recursive(world, self.entity);
    }
}

/// Trait that holds functions for droping recursively down the transform hierarchy
pub trait DropRecursiveExt {
    /// Drops the provided entity alongside all descendants.
    fn drop_recursive(self);

    /// Drops all descendants of the given entity.
    fn drop_descendants(&mut self) -> &mut Self;
}

impl<'w, 's, 'a> DropRecursiveExt for EntityCommands<'w, 's, 'a> {
    /// Drops the provided entity and its children.
    fn drop_recursive(mut self) {
        let entity = self.id();
        self.commands().add(DropRecursive { entity });
    }

    fn drop_descendants(&mut self) -> &mut Self {
        let entity = self.id();
        self.commands().add(DropChildrenRecursive { entity });
        self
    }
}

impl<'w> DropRecursiveExt for EntityWorldMut<'w> {
    /// Drops the provided entity and its children.
    fn drop_recursive(self) {
        let entity = self.id();

        #[cfg(feature = "trace")]
        let _span = bevy_utils::tracing::info_span!(
            "drop_recursive",
            entity = bevy_utils::tracing::field::debug(entity)
        )
        .entered();

        drop_with_children_recursive(self.into_world_mut(), entity);
    }

    fn drop_descendants(&mut self) -> &mut Self {
        let entity = self.id();

        #[cfg(feature = "trace")]
        let _span = bevy_utils::tracing::info_span!(
            "drop_descendants",
            entity = bevy_utils::tracing::field::debug(entity)
        )
        .entered();

        self.world_scope(|world| {
            drop_children_recursive(world, entity);
        });
        self
    }
}
