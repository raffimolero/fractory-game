use crate::{cam::MainCam, debug::Blocc, io::PlanetList};

use bevy::{prelude::*, sprite::Anchor, text::Text2dBounds, window::PrimaryWindow};
use fractory_common::sim::logic::{
    factory::FractoryMeta,
    planet::{BiomeId, PlanetId},
    presets::{XYYY, XYYY_LANDING_ZONE},
};

pub const SQRT_3: f32 = 1.732050807568877293527446341505872367_f32;
pub const TRI_SLOPE: f32 = SQRT_3;
pub const TRI_CIRC_R: f32 = SQRT_3 / 3.0;
pub const TRI_INSC_R: f32 = SQRT_3 / 6.0;
pub const TRI_CENTER_OFF_Y: f32 = SQRT_3 / 12.0;
pub const TRI_HEIGHT: f32 = SQRT_3 / 2.0;
pub const TRI_VERTS: [Vec2; 3] = [
    Vec2::new(-0.5, -TRI_INSC_R),
    Vec2::new(0.0, TRI_CIRC_R),
    Vec2::new(0.5, -TRI_INSC_R),
];

pub struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hover);
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct IsHovered(pub bool);

#[derive(Component, Debug, Clone, Copy)]
pub struct Hitbox {
    pub kind: HitboxKind,
    pub cursor: Option<CursorIcon>,
}

impl Default for Hitbox {
    fn default() -> Self {
        Self {
            kind: HitboxKind::Rect(Rect::new(-1.0, -1.0, 1.0, 1.0)),
            cursor: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HitboxKind {
    Rect(Rect),
    Tri { r: f32 },
}

impl HitboxKind {
    /// Whether this hitbox contains the point in question.
    ///
    /// If you're checking the mouse cursor, make sure the position is projected properly!
    /// - Object's GlobalTransform
    /// - Camera's GlobalTransform
    /// - Mouse's Window Position
    pub fn contains(&self, mut p: Vec2) -> bool {
        match self {
            HitboxKind::Rect(rect) => rect.contains(p),
            HitboxKind::Tri { r } => {
                let top = TRI_CIRC_R * *r - p.x.abs() * TRI_SLOPE;
                let bot = -TRI_INSC_R * *r;
                // dbg!(top, bot);
                (bot..top).contains(&p.y)
            }
        }
    }
}

fn hover(
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    camera: Query<(&GlobalTransform, &Camera), With<MainCam>>,
    mut hoverables: Query<(&GlobalTransform, &Hitbox, &mut IsHovered)>,
    mut commands: Commands,
    // mut gizmos: Gizmos,
) {
    let mut window = window.single_mut();
    let Some(pos) = window.cursor_position() else {
        return;
    };
    let Ok((cam_tf, cam)) = camera.get_single() else {
        return;
    };
    let Some(world_cursor_pos) = cam.viewport_to_world_2d(cam_tf, pos) else {
        return;
    };

    window.cursor.icon = CursorIcon::Default;
    let mut top = f32::MIN;
    for (gtf, hbx, mut hovered) in hoverables.iter_mut() {
        let projected_cursor = gtf
            .affine()
            .inverse()
            .transform_point3(world_cursor_pos.extend(0.0))
            .truncate();

        // let gizmo_scale = 200.0;
        // gizmos.circle_2d(projected_cursor * gizmo_scale, 5.0, Color::RED);
        // let verts = TRI_VERTS.map(|v| v * gizmo_scale);
        // for i in 0..3 {
        //     gizmos.line_2d(verts[i], verts[(i + 1) % 3], Color::ORANGE);
        // }
        // println!("{}", projected_cursor);

        if hbx.kind.contains(projected_cursor) {
            if !hovered.0 {
                hovered.0 = true;
            }
            let z = gtf.to_scale_rotation_translation().2.z;
            if z > top {
                top = z;
                if let Some(cursor) = hbx.cursor {
                    window.cursor.icon = cursor;
                }
            }
        } else {
            if hovered.0 {
                hovered.0 = false;
            }
        }
    }
}

// fn on_mouse_hover(mut commands: Commands, hoverables: Query<(&IsHovered, &mut OnMouseHover), Changed<IsHovered>>) {
//     hoverables.iter_mut()
// }
