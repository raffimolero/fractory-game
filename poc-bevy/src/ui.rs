//! Utility stuff for ui related stuff.

pub mod prelude {
    pub use super::{hover::*, state::*, *};
}

pub mod hover;
pub mod state;

use bevy::prelude::*;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugins((hover::Plug, state::Plug));
    }
}
