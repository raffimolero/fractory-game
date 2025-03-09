//! Utility stuff for ui related stuff.

pub mod prelude {
    pub use super::{elements::*, hover::prelude::*, state::prelude::*};
}

// pub mod despawn;
pub mod elements;
pub mod hover;
pub mod state;

use crate::prelude::*;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugins((hover::Plug, state::Plug));
    }
}
