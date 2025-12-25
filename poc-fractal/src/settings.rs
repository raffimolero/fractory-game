use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct KeyboardSensitivity {
    pub pan_sens: f32,
    pub rotate_sens: f32,
    pub zoom_sens: f32,
}

impl Default for KeyboardSensitivity {
    fn default() -> Self {
        Self {
            pan_sens: 4.0,
            rotate_sens: TAU / 2.0,
            zoom_sens: 4.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MouseSensitivity {
    pub zoom_sens: f32,
}

impl Default for MouseSensitivity {
    fn default() -> Self {
        Self {
            zoom_sens: 1.0 / 120.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Settings {
    // keybinds:
    pub mouse_sens: MouseSensitivity,
    pub keyboard_sens: KeyboardSensitivity,
}
