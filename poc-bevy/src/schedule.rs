use crate::prelude::*;

pub mod prelude {
    pub use super::{PostUpdateSet, StartupSet, UpdateSet};
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Startup,
            (StartupSet::Load, StartupSet::Cameras, StartupSet::Layout).chain(),
        )
        .configure_sets(
            Update,
            (
                UpdateSet::Input,
                UpdateSet::AnimationProgress,
                UpdateSet::OverrideProgress,
                UpdateSet::Animate,
                UpdateSet::Despawn,
                UpdateSet::Gui,
                UpdateSet::Logic,
            )
                .chain(),
        )
        // .configure_sets(
        //     PostUpdate,
        //     (
        //         PostUpdateSet::PreDespawn,
        //         PostUpdateSet::OnDespawn,
        //         PostUpdateSet::FinalDespawn,
        //     )
        //         .chain(),
        // )
        ;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, SystemSet)]
pub enum StartupSet {
    Load,
    Cameras,
    Layout,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, SystemSet)]
pub enum UpdateSet {
    Input,
    AnimationProgress,
    OverrideProgress,
    Animate,
    Despawn,
    Gui,
    Logic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, SystemSet)]
pub enum PostUpdateSet {}
