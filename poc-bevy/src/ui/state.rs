// TODO: reimplement bevy_tweening for zero fun and zero profit
// also maybe rename this file

use bevy::prelude::*;

pub mod prelude {
    pub use super::tween::Tween;
}

// pub struct Plug;
// impl Plugin for Plug {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Update);
//     }
// }

pub mod tween {
    pub trait Tween {
        fn tween(&self, value: f32) -> f32;
    }

    pub struct Linear;
    impl Tween for Linear {
        fn tween(&self, value: f32) -> f32 {
            value
        }
    }

    pub struct Quadratic;
    impl Tween for Quadratic {
        fn tween(&self, value: f32) -> f32 {
            value * value
        }
    }

    pub struct Cubic;
    impl Tween for Cubic {
        fn tween(&self, value: f32) -> f32 {
            value * value * value
        }
    }

    pub struct Quartic;
    impl Tween for Quartic {
        fn tween(&self, value: f32) -> f32 {
            value * value * value
        }
    }
}

pub mod lens {
    pub trait Lens<T> {
        fn lerp(&mut self, target: &mut T, ratio: f32);
    }
}
