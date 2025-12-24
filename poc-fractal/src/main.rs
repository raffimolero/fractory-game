#![allow(warnings)]

mod prelude;

pub mod ctx;
pub mod ui;

mod fractal_cam;
mod fractal_view_element;
mod fractory_element;
mod tile;

use prelude::*;

const DRAW_BRANCHES: bool = false;

/// used to catch accidental uses
#[allow(dead_code)]
fn apply(_youre_using_the_wrong_function: ()) {}

/// global singleton variables
struct Resources {
    planets: PlanetCache,
    biomes: BiomeCache,
}

impl Resources {
    fn new() -> Self {
        Self {
            planets: PlanetCache::default(),
            biomes: BiomeCache::default(),
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "WASD/Drag to move, Scroll to zoom, QE to rotate, F to flip.".to_owned(),
        fullscreen: true,
        window_resizable: false,
        ..Default::default()
    }
}

/// creates a new text tool, a function that draws text given a string
/// must be transformed and scaled into the appropriate position
fn new_text_tool(font: Font, color: Color) -> impl Fn(&str) {
    move |text| {
        let params = TextParams {
            font,
            font_size: 64,
            font_scale: 1.0 / 128.0,
            color,
            ..Default::default()
        };
        const SPACING: f32 = 0.7;
        let mut w = 0.0;
        let mut max_h = 0.0;
        let mut h = 0.0;

        for line in text.lines() {
            let dims = measure_text(line, Some(font), params.font_size, params.font_scale);
            w = dims.width.max(w);
            max_h = dims.height.max(max_h);
            h -= SPACING;
        }
        h += max_h;
        let x = (0.0 - w) / 2.0;
        let y = (0.5 + h) / 2.0;
        draw_multiline_text(text, x, y, SPACING, params)
    }
}

fn transform_to_mat4(transform: Transform) -> Mat4 {
    let mut matrix = Mat4::IDENTITY;
    if transform.reflected() {
        matrix = flip_x() * matrix;
    }
    let rot = transform.rotation() as u8 as f32 * TAU / 3.0;
    matrix = rotate_cw(rot) * matrix;
    matrix
}

fn tile_pos_to_mat4(tile_pos: TilePos) -> Mat4 {
    let off = Mat2::from_diagonal(Vec2::new(SIDE, HEIGHT))
        * (Mat3::from_cols_array_2d(&[
            [1.0, 0.0, 0.0],  //
            [-0.5, 1.0, 0.0], //
            [0.0, IN_R, 0.0], //
        ]) * Vec3::new(
            tile_pos.pos.x as f32,
            tile_pos.pos.y as f32,
            tile_pos.flop as u8 as f32,
        ))
        .truncate();
    let mat_scale = upscale(0.5_f32.powi(tile_pos.depth as i32));
    let flop = upscale(if tile_pos.flop { -1.0 } else { 1.0 });
    // HACK: shifting by out_r is required for displaying the floating tile
    mat_scale * shift(off.x, off.y) * shift(0.0, OUT_R) * flop
}

struct UiElement {
    font: Font,
    fractory: FractoryElement,
}

impl UiElement {
    fn new(res: &mut Resources, font: Font) -> Self {
        Self {
            font,
            fractory: FractoryElement::new(res),
        }
    }

    fn project_to_screen(ctx: &mut Context, f: impl FnOnce(&mut Context)) {
        let width = screen_width();
        let height = screen_height();
        let base_size = width.min(height);
        ctx.apply(
            scale(base_size / width as f32, base_size / height as f32),
            f,
        )
    }

    fn draw(&mut self, ctx: &mut Context, res: &mut Resources) {
        let text_tool = Box::new(new_text_tool(self.font, WHITE));
        let text_tool = ctx.register_text_tool(text_tool);
        Self::project_to_screen(ctx, |ctx| self.fractory.draw(ctx, res, text_tool));
    }

    fn input(&mut self, ctx: &mut Context, res: &mut Resources) {
        Self::project_to_screen(ctx, |ctx| self.fractory.input(ctx, res))
    }
}

/// holds the currently active biome data;
/// all the "base" fragments that exist in this instance
struct FractoryCache {
    fragments: FragmentData,
    biome: Biome,
}

/// NOTE: currently inside FractoryElement for whatever reason
enum CursorState {
    Free,
    Holding(Tile),
}

#[macroquad::main(window_conf)]
async fn main() {
    // camera for canvases
    let cam = &mut Camera2D::default();
    cam.zoom = Vec2::new(1.0, -1.0);
    set_camera(cam);

    // resource folder
    set_pc_assets_folder("../assets");
    // font
    let font = load_ttf_font("fonts/VarelaRound-Regular.ttf")
        .await
        .expect("rip varela round");

    let mut ctx = Context::default();
    let mut res = Resources::new();
    let mut ui_elem = UiElement::new(&mut res, font);

    let mut iters = 0;
    let mut time_check = Instant::now();
    // main loop
    loop {
        // Quit on Esc
        if is_key_pressed(KeyCode::Escape) {
            return;
        }

        ctx.update();
        ui_elem.input(&mut ctx, &mut res);
        ui_elem.draw(&mut ctx, &mut res);

        iters += 1;
        if iters >= 60 {
            println!("{iters} iters took {:?}", time_check.elapsed());
            iters = 0;
            time_check = Instant::now();
        }

        // end frame
        next_frame().await
    }
}
