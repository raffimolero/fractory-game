use crate::prelude::*;

mod draw;
mod input;
mod util;

#[derive(Debug)]
enum ViewState {
    Flat,
    Shattered,
}

impl ViewState {
    fn cycle(&mut self) {
        use ViewState::*;
        *self = match self {
            Flat => Shattered,
            Shattered => Flat,
        };
    }

    /// how much smaller each subtriangle should be;
    /// dictates how much margin there is between subtriangles,
    /// and dictates visibility of the parent triangle
    fn scaling(&self) -> f32 {
        use ViewState::*;
        match self {
            Flat => 1.0,
            Shattered => 1.0 - 2_f32.powi(-6),
        }
    }
}

pub struct FractalViewElement {
    pub(crate) view_state: ViewState,
    pub(crate) frac_cam: FractalCam,
}

impl FractalViewElement {
    pub fn new() -> Self {
        Self {
            view_state: ViewState::Shattered,
            frac_cam: FractalCam {
                camera: upscale(2.0) * shift(0.0, 0.625),
                ..Default::default()
            },
        }
    }
}
