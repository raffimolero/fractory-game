// TODO: reimplement bevy_tweening for zero fun and zero profit
// also maybe rename this file

use super::Despawn;

use bevy::prelude::*;

pub mod prelude {
    pub use super::{
        AnimationControl, AnimationEvents, AnimationProgress, AnimationTracker, AutoPause,
        ComponentAnimator, REvent, Tweener,
    };
}

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_controllers,
                update_progress,
                auto_pause,
                run_events,
                track_progress,
                (animate::<Transform>),
            )
                .chain(),
        );
    }
}

#[derive(Bundle, Default)]
pub struct AnimationControlBundle {
    pub control: AnimationControl,
    pub progress: AnimationProgress,
    pub events: AnimationEvents,
}

impl AnimationControlBundle {
    /// supports events happening before the start or after the end of the animation.
    ///
    /// allows tweens like back-in and back-out to work. maybe.
    pub fn from_events(
        duration_secs: f32,
        second_event_pairs: impl IntoIterator<Item = (f32, Box<dyn ReversibleEvent>)>,
    ) -> Self {
        if duration_secs <= 0.0 {
            panic!(
                "Durations cannot be zero or less.\n\
                The progress will only update if durations actually tick.\n\
                If you need an instant event, use f32::EPSILON."
            );
        }
        let progress_per_sec = 1.0 / duration_secs;
        let events = second_event_pairs
            .into_iter()
            .map(|(time, ev)| (time / duration_secs, ev))
            .collect::<Vec<_>>();
        for window in events.windows(2) {
            let &[(t1, _), (t2, _)] = window else {
                unreachable!()
            };
            assert!(t1 <= t2, "Animation events must be in order.");
        }
        Self {
            control: AnimationControl {
                playback_speed: 0.0,
                puppets: vec![],
                progress_per_sec,
            },
            progress: AnimationProgress(0.0),
            events: AnimationEvents {
                events,
                prev_progress: 0.0,
            },
        }
    }
}

#[derive(Bundle)]
pub struct AnimationPuppetBundle {
    pub progress: AnimationProgress,
    pub tracker: AnimationTracker,
}

impl AnimationPuppetBundle {
    pub fn track(controller: Entity) -> Self {
        Self {
            progress: default(),
            tracker: AnimationTracker(controller),
        }
    }
}

pub trait Tweener<T> {
    fn lerp(&mut self, target: &mut T, ratio: f32);
}

impl<T, F: FnMut(&mut T, f32)> Tweener<T> for F {
    fn lerp(&mut self, target: &mut T, ratio: f32) {
        self(target, ratio)
    }
}

#[derive(Component)]
pub struct AutoPause;

#[derive(Component)]
pub struct ComponentAnimator<T: Component>(pub Box<dyn Tweener<T> + Send + Sync>);

impl<T: Component> ComponentAnimator<T> {
    pub fn boxed(tweener: impl Tweener<T> + 'static + Send + Sync) -> Self {
        Self(Box::new(tweener))
    }
}

#[derive(Component, Default)]
pub struct AnimationProgress(pub f32);

pub trait ReversibleEvent: Send + Sync {
    fn run_forward(&mut self, commands: &mut Commands, puppets: &mut Vec<Entity>);
    fn run_backward(&mut self, commands: &mut Commands, puppets: &mut Vec<Entity>);
}

pub struct REvent<
    F: FnMut(&mut Commands, &mut Vec<Entity>) + 'static + Send + Sync,
    B: FnMut(&mut Commands, &mut Vec<Entity>) + 'static + Send + Sync,
> {
    pub fore: F,
    pub back: B,
}

impl<
        F: FnMut(&mut Commands, &mut Vec<Entity>) + 'static + Send + Sync,
        B: FnMut(&mut Commands, &mut Vec<Entity>) + 'static + Send + Sync,
    > REvent<F, B>
{
    pub fn boxed(fore: F, back: B) -> Box<dyn ReversibleEvent> {
        Box::new(Self { fore, back })
    }
}

impl<
        F: FnMut(&mut Commands, &mut Vec<Entity>) + Send + Sync,
        B: FnMut(&mut Commands, &mut Vec<Entity>) + Send + Sync,
    > ReversibleEvent for REvent<F, B>
{
    fn run_forward(&mut self, commands: &mut Commands, puppets: &mut Vec<Entity>) {
        (self.fore)(commands, puppets)
    }

    fn run_backward(&mut self, commands: &mut Commands, puppets: &mut Vec<Entity>) {
        (self.back)(commands, puppets)
    }
}

pub fn despawn_puppets(commands: &mut Commands, puppets: &mut Vec<Entity>) {
    for p in puppets.drain(..) {
        commands.entity(p).insert(Despawn);
    }
}

#[derive(Component, Default)]
pub struct AnimationEvents {
    events: Vec<(f32, Box<dyn ReversibleEvent>)>,
    prev_progress: f32,
}

impl AnimationEvents {
    fn update(&mut self, commands: &mut Commands, puppets: &mut Vec<Entity>, progress: f32) {
        if progress > self.prev_progress {
            self.events
                .iter_mut()
                .skip_while(|(p, _e)| *p < self.prev_progress)
                .take_while(|(p, _e)| *p <= progress)
                .for_each(|(_p, e)| e.run_forward(commands, puppets));
        } else if progress < self.prev_progress {
            self.events
                .iter_mut()
                .rev()
                .skip_while(|(p, _e)| *p > self.prev_progress)
                .take_while(|(p, _e)| *p >= progress)
                .for_each(|(_p, e)| e.run_backward(commands, puppets));
        }
        self.prev_progress = progress;
    }
}

#[derive(Component)]
pub struct AnimationTracker(pub Entity);

/// puppets must be spawned by AnimationEvents
#[derive(Component)]
pub struct AnimationControl {
    pub playback_speed: f32,
    pub puppets: Vec<Entity>,
    progress_per_sec: f32,
}

impl Default for AnimationControl {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationControl {
    pub fn new() -> Self {
        Self {
            playback_speed: 0.0,
            puppets: vec![],
            progress_per_sec: 0.0,
        }
    }
}

fn update_controllers(
    mut commands: Commands,
    mut animators: Query<(Entity, &mut AnimationControl), Changed<AnimationControl>>,
) {
    animators.for_each_mut(|(id, mut control)| {
        if control.playback_speed == 0.0 {
            for puppet in &mut control.puppets {
                commands.entity(*puppet).remove::<AnimationTracker>();
            }
        } else {
            for puppet in &mut control.puppets {
                commands.entity(*puppet).insert(AnimationTracker(id));
            }
        }
    });
}

fn update_progress(
    time: Res<Time>,
    mut animators: Query<(&AnimationControl, &mut AnimationProgress)>,
) {
    let delta = time.delta_seconds();
    animators.for_each_mut(|(control, mut progress)| {
        if control.playback_speed == 0.0 {
            return;
        }
        progress.0 += delta * control.progress_per_sec * control.playback_speed;
    })
}

fn auto_pause(
    mut animators: Query<
        (&mut AnimationControl, &mut AnimationProgress),
        (With<AutoPause>, Changed<AnimationProgress>),
    >,
) {
    animators.for_each_mut(|(mut control, mut progress)| {
        let clamped = progress.0.clamp(0.0, 1.0);
        if progress.0 != clamped {
            progress.0 = clamped;
            control.playback_speed = 0.0;
        }
    });
}

fn run_events(
    mut commands: Commands,
    mut animators: Query<(
        &mut AnimationControl,
        &AnimationProgress,
        &mut AnimationEvents,
    )>,
) {
    animators.for_each_mut(|(mut control, progress, mut events)| {
        events.update(&mut commands, &mut control.puppets, progress.0);
    });
}

fn track_progress(
    mut trackers: Query<(&AnimationTracker, &mut AnimationProgress)>,
    tracked: Query<&AnimationProgress, Without<AnimationTracker>>,
) {
    trackers.for_each_mut(|(tracker, mut progress)| {
        if let Ok(tracked) = tracked.get(tracker.0) {
            if progress.0 != tracked.0 {
                progress.0 = tracked.0;
            }
        }
    })
}

pub fn animate<T: Component>(
    mut animators: Query<
        (&mut ComponentAnimator<T>, &mut T, &AnimationProgress),
        Changed<AnimationProgress>,
    >,
) {
    animators.for_each_mut(|(mut animator, mut target, progress)| {
        animator.0.lerp(&mut target, progress.0);
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
