use fractory_common::sim::logic::orientation::Symmetries;

use crate::prelude::*;

/// std::f32::consts::SQRT_3 is unstable so here it is
const SQRT_3: f32 = 1.732050807568877293527446341505872367_f32;

pub const SIDE: f32 = SCREEN_SPACE_WIDTH;
pub const HALF_SIDE: f32 = SIDE / 2.0;
pub const HEIGHT: f32 = SIDE * SQRT_3 / 2.0;
pub const OUT_R: f32 = SIDE * SQRT_3 / 3.0;
pub const IN_R: f32 = OUT_R / 2.0;

pub const TRIANGLE: [Vec2; 3] = {
    [
        Vec2 {
            x: -HALF_SIDE,
            y: IN_R,
        },
        Vec2 {
            x: HALF_SIDE,
            y: IN_R,
        },
        Vec2 { x: 0.0, y: -OUT_R },
    ]
};

pub fn in_triangle(Vec2 { x, y }: Vec2) -> bool {
    let slope = SQRT_3;
    let x = x.abs();

    let top = -OUT_R + (x * slope);
    let bot = IN_R;

    (top..bot).contains(&y)
}

pub fn triangle_transforms() -> [Mat4; 4] {
    [
        flip_xy(),
        shift(0.0, -OUT_R),
        shift(HALF_SIDE, IN_R),
        shift(-HALF_SIDE, IN_R),
    ]
    .map(|t| downscale(2.0) * t)
}

pub fn tile_symmetries(fractory: &Fractory, tile_id: usize) -> Symmetries {
    fractory.fractal.library[tile_id].symmetries
}
pub fn tile_color(fractory: &Fractory, tile_id: usize) -> Color {
    enum ColorMode {
        Fragment,
        Id,
        Greyscale,
    }
    use ColorMode::*;

    fn average(a: Color, b: Color) -> Color {
        Color {
            r: a.r + b.r / 2.0,
            g: a.g + b.g / 2.0,
            b: a.b + b.b / 2.0,
            a: a.a + b.a / 2.0,
        }
    }

    let SlotInfo {
        quad,
        fill,
        symmetries: _,
    } = fractory.fractal.library[tile_id];

    let color_mode = match fill {
        TileFill::Empty => Greyscale,
        TileFill::Partial => Id,
        TileFill::Full | TileFill::Leaf => Fragment,
    };

    match color_mode {
        Id => {
            const PALETTE: &[Color] = &[RED, ORANGE, GOLD, GREEN, BLUE, PURPLE];
            average(BLACK, PALETTE[tile_id % PALETTE.len()])
        }
        Fragment => {
            const PALETTE: &[Color] = &[RED, ORANGE, GOLD, GREEN, BLUE, PURPLE];
            PALETTE[tile_id % PALETTE.len()]
        }
        Greyscale => {
            // const PALETTE: &[Color] = &[DARKGRAY, GRAY, LIGHTGRAY];
            // PALETTE[pos.depth() % PALETTE.len()]
            DARKGRAY
        }
    }
}

pub fn tile_name(names: &[String], tile_id: usize) -> String {
    match names.get(tile_id) {
        Some(name) => name.clone(),
        None => tile_id.to_string(),
    }
}

pub enum TileStyle {
    Plain,
    Bordered {
        border_color: Color,
        orient_icon: Option<Symmetries>,
    },
}

pub fn draw_tile(
    ctx: &mut Context,
    text_tool: TextToolId,
    color: Color,
    name: String,
    style: TileStyle,
) {
    if let TileStyle::Bordered {
        border_color,
        orient_icon,
    } = style
    {
        ctx.queue_polygon(&TRIANGLE, border_color);
        ctx.apply(upscale(0.8), |ctx| {
            ctx.queue_polygon(&TRIANGLE, color);
        });
        if let Some(sym) = orient_icon {
            let iters = if sym.is_rotational() { 3 } else { 1 };
            let tri_rot = if sym.is_reflectional() {
                0.0
            } else {
                TAU / 4.0
            };
            for i in 0..iters {
                ctx.apply(
                    rotate_cw(TAU / 3.0 * i as f32)
                        * shift(0.0, -0.625)
                        * downscale(8.0)
                        * rotate_cw(tri_rot),
                    |ctx| {
                        ctx.queue_polygon(&TRIANGLE, border_color);
                    },
                )
            }
        }
    } else {
        ctx.queue_polygon(&TRIANGLE, color);
    }

    let scale = 0.5 / name.len() as f32 + 0.5;
    ctx.apply(upscale(scale), |ctx| ctx.queue_text(text_tool, name));
}
