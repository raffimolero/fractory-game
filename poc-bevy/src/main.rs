mod cam;
pub mod debug;
mod fractal;
mod io;
mod ui;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((bevy_tweening::TweeningPlugin, io::Plug))
        .add_plugins((cam::Plug, debug::Plug, fractal::Plug, ui::Plug))
        .add_systems(Update, bevy::window::close_on_esc)
        .run()
}
