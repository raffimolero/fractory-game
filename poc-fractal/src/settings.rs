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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KeyboardControlMode {
    #[default]
    Smooth,
    Snap,
}

impl KeyboardControlMode {
    pub fn toggle(&mut self) {
        use KeyboardControlMode::*;
        *self = match self {
            Smooth => Snap,
            Snap => Smooth,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Settings {
    // keybinds:
    pub mouse_sens: MouseSensitivity,
    pub keyboard_sens: KeyboardSensitivity,
    pub keyboard_control_mode: KeyboardControlMode,
}
