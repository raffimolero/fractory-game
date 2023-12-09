mod cam;
pub mod debug;
mod fractal;
mod io;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(io::Plug)
        .add_plugins((
            cam::Plug,
            // debug::Plug,
            fractal::Plug,
        ))
        .add_systems(Update, bevy::window::close_on_esc)
        .run()
}
