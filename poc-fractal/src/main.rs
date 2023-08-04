#![allow(warnings)]

// TODO: use Affine2 instead of Mat4

mod tree;

use self::ctx::Context;
use fractory_common::sim::logic::path::{SubTile, TilePos};
use fractory_common::sim::logic::{fractal::Fractal, tile::Tile};
use std::f32::consts::TAU;

// use ::rand::prelude::*; // NOTE: ergoquad::prelude::rand exists, and is from macroquad
use ergoquad_2d::macroquad; // NOTE: ergoquad2d does not provide its own macro
use ergoquad_2d::prelude::*;

/// used to catch accidental uses
#[allow(dead_code)]
fn apply(_youre_using_the_wrong_function: ()) {}

/// returns a Mat4 corresponding to how much the map needs to be moved
fn cam_control() -> Mat4 {
    let [mut x, mut y, mut rot] = [0.0; 3];
    let mut flipped = false;
    let mut zoom = 1.0;

    // // nearly every macroquad function uses f32 instead of f64 because that's what `Mat4`s are made of
    // let time = get_time() as f32;
    // for some reason this uses f32s already
    let delta = get_frame_time();

    // check mouse
    // mouse goes downwards, while transforms go upwards
    // if the mouse hasn't moved since startup, this will be 0, 0
    let mut mouse = mouse_position_local();
    // if mouse out of bounds, default to center of screen
    if !(-1.0..1.0).contains(&mouse.x) || !(-1.0..1.0).contains(&mouse.y) {
        mouse = Vec2::ZERO;
    }

    // macroquad calculates delta position wrong, because macroquad
    let mouse_delta = -mouse_delta_position();

    // scroll goes up, transforms zoom in
    let (_scroll_x, scroll_y) = mouse_wheel();
    {
        // zoom
        let scroll_sens = 1.0 / 120.0;
        zoom *= (2_f32).powf(scroll_y * scroll_sens);

        // drag controls
        if is_mouse_button_down(MouseButton::Right) {
            x += mouse_delta.x;
            y += mouse_delta.y;
        }
    }

    // check keypresses
    {
        use KeyCode::*;

        // zoom
        let zoom_sens = 4.0;
        if is_key_down(LeftShift) {
            zoom *= (2_f32).powf(delta * zoom_sens);
        }
        if is_key_down(Space) {
            zoom /= (2_f32).powf(delta * zoom_sens);
        }

        let speed = 4.0;
        // WASD movement, y goes down
        if is_key_down(W) {
            y += delta * speed;
        }
        if is_key_down(S) {
            y -= delta * speed;
        }
        if is_key_down(A) {
            x += delta * speed;
        }
        if is_key_down(D) {
            x -= delta * speed;
        }

        // rotation, clockwise
        let sensitivity = TAU / 2.0; // no i will not use pi
        if is_key_down(Q) {
            rot -= delta * sensitivity;
        }
        if is_key_down(E) {
            rot += delta * sensitivity;
        }

        if is_key_pressed(F) {
            flipped ^= true;
        }
    }

    let main_transform = Mat4::from_scale_rotation_translation(
        Vec3 {
            x: if flipped { -zoom } else { zoom },
            y: zoom,
            z: 1.0,
        },
        Quat::from_rotation_z(rot),
        Vec3 { x, y, z: 0.0 },
    );

    // center the transform at the mouse
    shift(mouse.x, mouse.y) * main_transform * shift(-mouse.x, -mouse.y)
}

fn window_conf() -> Conf {
    Conf {
        window_title: "WASD/Drag to move, Scroll to zoom, QE to rotate, F to flip.".to_owned(),
        window_width: 512,
        window_height: 512,
        fullscreen: false,
        window_resizable: true,
        ..Default::default()
    }
}

fn new_text_tool(font: Font, color: Color) -> impl Fn(&str) {
    move |text| {
        let params = TextParams {
            font,
            font_size: 64,
            font_scale: 1.0 / 128.0,
            color,
            ..Default::default()
        };
        let dims = measure_text(text, Some(font), params.font_size, params.font_scale);
        draw_text_ex(
            text,
            (0.0 - dims.width) / 2.0,
            (0.5 + dims.height) / 2.0,
            params,
        )
    }
}

mod ctx {
    use super::*;

    #[derive(Default)]
    pub struct Context {
        mouse: Option<Vec2>,
        last_lmb: Option<Vec2>,
        last_rmb: Option<Vec2>,
        matrix: Mat4,
        inv_matrix: Mat4,
    }

    impl Context {
        pub fn apply(&mut self, matrix: Mat4, f: impl FnOnce(&mut Self)) {
            let orig = (self.matrix, self.inv_matrix);
            self.matrix = self.matrix * matrix;
            self.inv_matrix = self.matrix.inverse();
            ergoquad_2d::prelude::apply(matrix, || f(self));
            (self.matrix, self.inv_matrix) = orig;
        }

        fn project(&self, point: Option<Vec2>) -> Option<Vec2> {
            point.map(|pos| self.inv_matrix.transform_point3(pos.extend(0.0)).truncate())
        }

        pub fn mouse_pos(&self) -> Option<Vec2> {
            self.project(self.mouse)
        }

        pub fn lmb_pos(&self) -> Option<Vec2> {
            self.project(self.last_lmb)
        }

        pub fn rmb_pos(&self) -> Option<Vec2> {
            self.project(self.last_rmb)
        }

        pub fn update(&mut self) {
            fn detect_click(
                cur_mouse_pos: Option<Vec2>,
                click_pos: &mut Option<Vec2>,
                btn: MouseButton,
            ) {
                if is_mouse_button_pressed(btn) {
                    *click_pos = cur_mouse_pos;
                }
            }

            self.mouse.replace(mouse_position_local());
            if is_mouse_button_pressed(MouseButton::Left) {
                self.last_lmb = self.mouse;
            }
            if is_mouse_button_pressed(MouseButton::Right) {
                self.last_rmb = self.mouse;
            }
        }
    }
}

#[derive(Debug)]
enum UiState {
    View,
    Toggle,
}

impl UiState {
    fn cycle(&mut self) {
        use UiState::*;
        *self = match self {
            View => Toggle,
            Toggle => View,
        };
    }

    /// how much smaller each subtriangle should be;
    /// dictates how much margin there is between subtriangles,
    /// and dictates visibility of the parent triangle
    fn scaling(&self) -> f32 {
        use UiState::*;
        match self {
            View => 1.0,
            Toggle => 0.9,
        }
    }
}

struct TreeElement {
    // this should eventually be replaced by handles to real images
    // which would all be contained in Fragment data
    font: Font,
    ui_state: UiState,

    camera: Mat4,

    fractal: Fractal,
    max_depth: usize,
}

impl TreeElement {
    fn new(font: Font) -> Self {
        Self {
            font,
            ui_state: UiState::View,

            camera: Mat4::IDENTITY,

            fractal: Fractal::new_binary(),
            max_depth: 3,
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        let text_tool = new_text_tool(self.font, WHITE);
        ctx.apply(self.camera, |ctx| {
            let mut draw_num = |ctx: &mut Context, number: usize| {
                ctx.apply(shift(0.0, -0.3), |_| {
                    text_tool(&number.to_string());
                })
            };
            draw_tree(
                ctx,
                &self.fractal,
                self.max_depth,
                self.ui_state.scaling(),
                &mut draw_num,
            );
        });

        ctx.apply(shift(0.0, 0.65) * downscale(5.0), |_ctx| {
            text_tool(&format!("Mode: {:?}", self.ui_state))
        });
        ctx.apply(shift(0.0, 0.8) * downscale(5.0), |_ctx| {
            text_tool("press Tab to cycle between modes")
        });
    }

    fn input_view(&mut self, ctx: &mut Context) {
        // nothing
    }

    fn click_tree(&mut self, click: Vec2, depth: usize) -> Option<TilePos> {
        dbg!(depth);
        if depth > self.max_depth {
            return None;
        }
        if !in_triangle(click) {
            return None;
        }

        let w = 1.0;
        let side = 2.0;
        let out_r = 3_f32.sqrt() / 3.0 * side;
        let in_r = out_r / 2.0;

        let transforms = [
            flip_xy(),
            shift(0.0, -out_r),
            shift(w, in_r),
            shift(-w, in_r),
        ]
        .map(|t| downscale(2.0) * t * downscale(1.1))
        .map(|t| t.inverse());

        for (transform, subtile) in transforms.into_iter().zip(SubTile::ORDER) {
            let hit_pos = self.click_tree(
                transform.transform_point3(click.extend(0.0)).truncate(),
                depth + 1,
            );
            if let Some(mut tile_pos) = hit_pos {
                tile_pos.push_front(dbg!(subtile));
                return Some(tile_pos);
            }
        }
        Some(TilePos::UNIT)
    }

    fn input_toggle(&mut self, ctx: &mut Context) {
        if is_mouse_button_released(MouseButton::Left) {
            // compute if the mouse wasn't dragged too far away
            // if the mouse is clicked and dragged further than the leash range,
            // then actions should be cancelled.
            let screen_width = 2.0;
            let leash = screen_width / 128.0;
            let leash_sq = leash * leash;

            let Some(down) = ctx.lmb_pos() else {
                return;
            };
            let Some(up) = ctx.mouse_pos() else {
                return;
            };
            let in_range = (down - up).length_squared() < leash_sq;

            let pos = self.camera.transform_point3(down.extend(0.0)).truncate();
            if in_range && in_triangle(down) {
                if let Some(hit_pos) = self.click_tree(pos, 0) {
                    dbg!(hit_pos);
                    let mut tile = self.fractal.get(hit_pos);
                    tile.id = 1 - tile.id;
                    self.fractal.set(hit_pos, tile);
                }
            }
        }
    }

    fn input(&mut self, ctx: &mut Context) {
        self.camera = cam_control() * self.camera;
        if is_key_pressed(KeyCode::Tab) {
            self.ui_state.cycle();
        }

        match self.ui_state {
            UiState::View => self.input_view(ctx),
            UiState::Toggle => self.input_toggle(ctx),
        }
    }
}

fn in_triangle(Vec2 { x, y }: Vec2) -> bool {
    let side = 2.0;
    let out_r = 3_f32.sqrt() / 3.0 * side;
    let in_r = out_r / 2.0;
    let slope = 3_f32.sqrt();

    let x = x.abs();

    let top = -out_r + (x * slope);
    let bot = in_r;

    (top..bot).contains(&y)
}

fn draw_tree(
    ctx: &mut Context,
    fractal: &Fractal,
    max_depth: usize,
    scaling: f32,
    draw_leaf: &mut impl FnMut(&mut Context, usize),
) {
    // TODO: add a way to test the fractal

    fn inner(
        ctx: &mut Context,
        fractal: &Fractal,
        tile: Tile,   // changes
        depth: usize, // changes
        max_depth: usize,
        scaling: f32,
        draw_leaf: &mut impl FnMut(&mut Context, usize),
    ) {
        if tile.id == 0 || depth > max_depth {
            return;
        }
        let (quad, info) = fractal.library[tile.id];

        let w = 1.0;
        let side = 2.0;
        let out_r = 3_f32.sqrt() / 3.0 * side;
        let in_r = out_r / 2.0;

        let mut draw_tringle = |ctx: &mut Context, number, color| {
            draw_triangle(
                Vec2 { x: -1.0, y: in_r },
                Vec2 { x: 1.0, y: in_r },
                Vec2 { x: 0.0, y: -out_r },
                color,
            );
            draw_leaf(ctx, number);
        };

        let mouse = ctx.mouse_pos().unwrap_or(Vec2::ZERO);
        if info.is_full && in_triangle(mouse) {
            const PALETTE: &[Color] = &[RED, ORANGE, YELLOW, GREEN, BLUE, PURPLE];
            let col = PALETTE[depth % PALETTE.len()];
            draw_tringle(ctx, tile.id, col);
        } else {
            draw_tringle(ctx, tile.id, DARKGRAY);
        }

        let transforms = [
            flip_xy(),
            shift(0.0, -out_r),
            shift(w, in_r),
            shift(-w, in_r),
        ]
        .map(|t| downscale(2.0) * t * upscale(scaling));

        for (transform, tile) in transforms.into_iter().zip(quad.0) {
            ctx.apply(transform, |ctx| {
                inner(ctx, fractal, tile, depth + 1, max_depth, scaling, draw_leaf);
            });
        }
    }
    inner(ctx, fractal, fractal.root, 0, max_depth, scaling, draw_leaf);
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
    let mut tree_elem = TreeElement::new(font);

    // main loop
    loop {
        // Quit on Esc
        if is_key_pressed(KeyCode::Escape) {
            return;
        }

        ctx.update();
        tree_elem.input(&mut ctx);
        tree_elem.draw(&mut ctx);

        // end frame
        next_frame().await
    }
}
