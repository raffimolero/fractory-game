use bevy::prelude::*;
fn main() {
    App::new()
        .add_startup_system(setup)
        .add_system(tick)
        .run()
}

fn tick() {

}

fn setup(mut commands: Commands) {
    println!("Hello, World!");
}
