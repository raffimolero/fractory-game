// TODO: reimplement bevy_tweening for zero fun and zero profit
// also maybe rename this file

use std::time::{Duration, Instant};

use bevy::prelude::*;

pub mod prelude {
    // TODO
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_progress,
                update_controllers,
                track_progress,
                (animate::<Transform>),
            )
                .chain(),
        );
    }
}

pub trait Tweener<T> {
    fn lerp(&mut self, target: &mut T, ratio: f32);
}

// pub struct Keyframes<T> {
//     tweens: Vec<(Progress, Box<dyn Tweener<T>>)>,
//     prev_keyframe: usize,
//     prev_progress: Progress,
// }

// impl<T> Tweener<T> for Keyframes<T> {
//     fn lerp(&mut self, target: &mut T, ratio: Progress) {
//         if self.prev_progress > ratio {
//             self.keyframes[self.prev_keyframe + 1].0
//         }
//     }
// }

#[derive(Component)]
pub struct ComponentAnimator<T: Component>(pub Box<dyn Tweener<T> + Send + Sync>);

#[derive(Component)]
pub struct AnimationProgress(f32);

pub trait ReversibleEvent: Send + Sync {
    fn forward(&mut self, commands: &mut Commands);
    fn backward(&mut self, commands: &mut Commands);
}

#[derive(Component)]
pub struct AnimationEvents {
    events: Vec<(f32, Box<dyn ReversibleEvent>)>,
    prev_progress: f32,
}

impl AnimationEvents {
    fn update(&mut self, commands: &mut Commands, progress: f32) {
        if progress > self.prev_progress {
            self.events
                .iter_mut()
                .skip_while(|(p, _e)| *p < self.prev_progress)
                .take_while(|(p, _e)| *p < progress)
                .for_each(|(_p, e)| e.forward(commands));
        } else {
            self.events
                .iter_mut()
                .rev()
                .skip_while(|(p, _e)| *p >= self.prev_progress)
                .take_while(|(p, _e)| *p >= progress)
                .for_each(|(_p, e)| e.backward(commands));
        }
    }
}

#[derive(Component)]
pub struct AnimationTracker(Entity);

/// puppets must be spawned by AnimationEvents
#[derive(Component)]
pub struct AnimationControl {
    pub playback_speed: f32,
    pub puppets: Vec<Entity>,
    progress_per_sec: f32,
}

fn update_controllers(
    mut commands: Commands,
    mut animators: Query<(Entity, &mut AnimationControl), Changed<AnimationControl>>,
) {
    animators.for_each_mut(|(id, mut control)| {
        if control.playback_speed == 0.0 {
            for puppet in &mut control.puppets {
                commands.entity(*puppet).insert(AnimationTracker(id));
            }
        } else {
            for puppet in &mut control.puppets {
                commands.entity(*puppet).remove::<AnimationTracker>();
            }
        }
    });
}

fn update_progress(
    mut commands: Commands,
    mut animators: Query<(
        &mut AnimationControl,
        &mut AnimationProgress,
        Option<&mut AnimationEvents>,
    )>,
) {
    animators.for_each_mut(|(mut control, mut progress, events)| {
        progress.0 += control.progress_per_sec * control.playback_speed;
        let clamped = progress.0.clamp(0.0, 1.0);
        if progress.0 != clamped {
            progress.0 = clamped;
            control.playback_speed = 0.0;
        }
        if let Some(mut events) = events {
            events.update(&mut commands, progress.0);
        }
    })
}

fn track_progress(
    mut progressors: Query<(&AnimationTracker, &mut AnimationProgress)>,
    tracked: Query<&AnimationProgress, Without<AnimationTracker>>,
) {
    progressors.for_each_mut(|(tracker, mut progress)| {
        if let Ok(tracked) = tracked.get(tracker.0) {
            progress.0 = tracked.0;
        }
    })
}

fn animate<T: Component>(
    mut animators: Query<
        (&mut ComponentAnimator<T>, &mut T, &AnimationProgress),
        Changed<AnimationProgress>,
    >,
) {
    animators.for_each_mut(|(mut animator, mut target, progress)| {
        animator.0.lerp(target.as_mut(), progress.0);
    })
}

pub mod tween {
    use super::Tweener;

    pub struct Linear;
    impl Tweener<f32> for Linear {
        fn lerp(&mut self, target: &mut f32, ratio: f32) {
            *target = ratio;
        }
    }

    pub struct Quadratic;
    impl Tweener<f32> for Quadratic {
        fn lerp(&mut self, target: &mut f32, ratio: f32) {
            *target = ratio * ratio;
        }
    }

    pub struct Cubic;
    impl Tweener<f32> for Cubic {
        fn lerp(&mut self, target: &mut f32, ratio: f32) {
            *target = ratio * ratio * ratio;
        }
    }

    pub struct Quartic;
    impl Tweener<f32> for Quartic {
        fn lerp(&mut self, target: &mut f32, ratio: f32) {
            *target = ratio * ratio * ratio * ratio;
        }
    }
}

pub mod lens {
    pub trait Lens<T> {
        fn lerp(&mut self, target: &mut T, ratio: f32);
    }
}
