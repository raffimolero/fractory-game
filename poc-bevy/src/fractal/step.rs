use bevy::prelude::*;
use fractory_common::sim::logic::actions::CleanMoveList;

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {}
}

// pub struct Step {
//     move_list: CleanMoveList,
// }

// impl Command for Step {
//     fn apply(self, world: &mut World) {
//         // Commands
//         // apply_deferred()
//         for pos in self.move_list.moves() {
//             todo!();
//         }
//     }
// }
