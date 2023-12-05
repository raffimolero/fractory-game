#![allow(warnings)]

mod ctx;

const DRAW_BRANCHES: bool = false;

// TODO: use Affine2 instead of Mat4

use self::ctx::{Click, Context};
use ctx::TextToolId;
use fractory_common::sim::logic::{
    factory::{ActiveTiles, Fractory, FractoryMeta},
    fractal::{Fractal, SlotInfo, TileFill},
    orientation::{Orient, Rotation, Transform},
    path::TilePos,
    planet::{Behavior, Biome, Filter, FragmentData, Planet, PlanetCache},
    tile::{SubTile, Tile},
};
use std::{
    f32::consts::TAU,
    ops::{ControlFlow, Mul},
    time::{Duration, Instant},
};

// use ::rand::prelude::*; // NOTE: ergoquad::prelude::rand exists, and is from macroquad
use ergoquad_2d::macroquad; // NOTE: ergoquad2d does not provide its own macro
use ergoquad_2d::prelude::*;

/// used to catch accidental uses
#[allow(dead_code)]
fn apply(_youre_using_the_wrong_function: ()) {}

const TRIANGLE: [Vec2; 3] = {
    let side = 2.0;
    /// std::f32::consts::SQRT_3 is unstable so here it is
    const SQRT_3: f32 = 1.732050807568877293527446341505872367_f32;
    let out_r = SQRT_3 / 3.0 * side;
    let in_r = out_r / 2.0;

    [
        Vec2 { x: -1.0, y: in_r },
        Vec2 { x: 1.0, y: in_r },
        Vec2 { x: 0.0, y: -out_r },
    ]
};

struct Resources {
    planets: PlanetCache,
}

impl Resources {
    fn new() -> Self {
        Self {
            planets: PlanetCache::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct FractalCam {
    camera: Mat4,
    min_depth: f32,
    min_bg_depth: f32,
    mouse_depth: f32,
    max_bg_depth: f32,
    max_mouse_depth: f32,
}

impl Default for FractalCam {
    fn default() -> Self {
        Self {
            camera: Mat4::IDENTITY,
            min_depth: -1.0,
            min_bg_depth: 1.0,
            mouse_depth: 4.0,
            max_bg_depth: 4.0,
            max_mouse_depth: 6.0,
        }
    }
}

impl FractalCam {
    /// returns a Mat4 corresponding to how much the map needs to be moved
    fn input(ctx: &Context) -> Self {
        use KeyCode::*;

        let [mut x, mut y, mut rot] = [0.0; 3];
        let mut flipped = false;
        let mut zoom = 1.0;
        let mut min_bg_depth = 0.0;
        let mut mouse_depth = 0.0;

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

        let mouse_zoom = {
            mouse = ctx.project(mouse);
            let mouse_delta = ctx.project(-mouse_delta_position());
            // scroll goes up, transforms zoom in
            let (_scroll_x, scroll_y) = mouse_wheel();

            // drag controls
            if is_mouse_button_down(MouseButton::Left) | is_mouse_button_down(MouseButton::Right) {
                x += mouse_delta.x;
                y += mouse_delta.y;
            }

            // zoom
            let scroll_sens = 1.0 / 120.0;
            let zoom_amount = scroll_y * scroll_sens;
            zoom_amount
        };

        // check keypresses
        let keyboard_zoom = {
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
            // zoom
            if is_key_down(Space) {
                let zoom_sens = 4.0;
                let zoom_sign = if is_key_down(LeftShift) { -1.0 } else { 1.0 };
                let zoom_amount = delta * zoom_sens * zoom_sign;
                zoom_amount
            } else {
                0.0
            }
        };

        let zoom_amount = mouse_zoom + keyboard_zoom;
        let mut zoom_scaling = (2_f32).powf(zoom_amount);
        if is_key_down(LeftControl) {
            mouse_depth += zoom_amount;
        } else if is_key_down(LeftAlt) {
            min_bg_depth += zoom_amount;
        } else {
            mouse_depth -= zoom_amount;
            zoom *= zoom_scaling;
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
        let camera = shift(mouse.x, mouse.y) * main_transform * shift(-mouse.x, -mouse.y);
        FractalCam {
            camera,
            min_depth: 0.0,
            min_bg_depth,
            mouse_depth,
            max_bg_depth: 0.0,
            max_mouse_depth: 0.0,
        }
    }

    fn scale(&self) -> f32 {
        let (scale, _, _) = self.camera.to_scale_rotation_translation();
        scale.y // scale.x.abs() == scale.y
    }

    fn min_depth(&self) -> usize {
        (self.scale().log2() + self.min_bg_depth) as usize
    }

    fn hover_depth(&self) -> usize {
        (self.scale().log2() + self.mouse_depth) as usize
    }

    fn max_depth(&self) -> usize {
        (self.scale().log2() + self.max_bg_depth) as usize
    }

    fn clamp_depth(self) -> Self {
        let scale = self.scale();
        let min = self.min_depth;
        let max = self.max_mouse_depth;
        let mouse_depth = self.mouse_depth.clamp(min, max);

        let min = self.min_depth;
        let max = self.max_bg_depth;
        let min_bg_depth = self.min_bg_depth.clamp(min, max);

        Self {
            min_bg_depth,
            mouse_depth,
            ..self
        }
    }
}

impl Mul for FractalCam {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            camera: self.camera * rhs.camera,
            min_depth: self.min_depth + rhs.min_depth,
            min_bg_depth: self.min_bg_depth + rhs.min_bg_depth,
            mouse_depth: self.mouse_depth + rhs.mouse_depth,
            max_bg_depth: self.max_bg_depth + rhs.max_bg_depth,
            max_mouse_depth: self.max_mouse_depth + rhs.max_mouse_depth,
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

// TODO: -> impl Fn(Size, std::fmt::Alignment, &str) {}
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
        draw_multiline_text(text, (0.0 - w) / 2.0, (0.5 + h) / 2.0, SPACING, params)
    }
}

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
        Self::project_to_screen(ctx, |ctx| self.fractory.draw(ctx, res, text_tool))
    }

    fn input(&mut self, ctx: &mut Context, res: &mut Resources) {
        Self::project_to_screen(ctx, |ctx| self.fractory.input(ctx, res))
    }
}

struct FractoryCache {
    fragments: FragmentData,
    biome: Biome,
}

struct FractoryElement {
    fractory_meta: FractoryMeta,
    fractal_view: FractalViewElement,
    // inventory_view: InventoryViewElement,
    cache: FractoryCache,
}

impl FractoryElement {
    fn new(res: &mut Resources) -> Self {
        let fractory_meta = FractoryMeta::new_xyyy(&mut res.planets);
        let planet = res.planets.get(&fractory_meta.planet).unwrap();
        let fragments = planet.fragments();
        let biome = planet.biomes().get(&fractory_meta.biome).unwrap();
        let cache = FractoryCache {
            fragments: fragments.to_owned(),
            biome: biome.to_owned(),
        };
        Self {
            fractory_meta,
            fractal_view: FractalViewElement::new(),
            cache,
        }
    }

    fn draw(&mut self, ctx: &mut Context, res: &mut Resources, text_tool: TextToolId) {
        self.draw_inventory(ctx, text_tool);
        self.fractal_view
            .draw(ctx, res, &self.fractory_meta, &self.cache, text_tool);

        ctx.apply(shift(0.0, 0.6) * downscale(10.0), |ctx| {
            ctx.queue_text(
                text_tool,
                "Esc: quit\n\
                Tab: toggle shattered view\n\
                Enter: tick\n\
                Camera:\n\
                -> WASD: move | Q/E: rotate | F: flip | (Shift+)Space: zoom (out)in\n\
                -> Click+Drag: move | Scroll: zoom | (Ctrl/Alt)+Scroll: change cursor/background depth\n\
                Shift+LMB/RMB: Rotate tile (no effect on rotational tiles such as X, Y, Rotor)\n\
                Ctrl+LMB: Activate tile | Ctrl+RMB: Flip tile (no effect on reflective tiles)\n\
                Ctrl+Shift+LMB/RMB: Cycle tile\n\
                *Some edits may change other tiles' rotations. This is normal."
                    .into(),
            );
        });
    }

    fn draw_inventory(&mut self, ctx: &mut Context, text_tool: TextToolId) {
        let wrap = (self.fractory_meta.fractory.inventory.len() as f32).sqrt() as usize;
        for (idx, (tile_id, count)) in self.fractory_meta.fractory.inventory.iter().enumerate() {
            let x = idx % wrap;
            let y = idx / wrap;
        }
    }

    fn input(&mut self, ctx: &mut Context, res: &mut Resources) {
        self.fractal_view
            .input(ctx, res, &mut self.fractory_meta.fractory, &self.cache);
    }
}

struct FractalViewElement {
    view_state: ViewState,
    frac_cam: FractalCam,
}

impl FractalViewElement {
    fn new() -> Self {
        Self {
            view_state: ViewState::Shattered,
            frac_cam: FractalCam {
                camera: upscale(2.0) * shift(0.0, 0.625),
                ..Default::default()
            },
        }
    }

    fn draw_leaf(
        &self,
        ctx: &mut Context,
        fractory: &Fractory,
        cache: &FractoryCache,
        names: &[String],
        id: usize,
        tile_fill: TileFill,
        pos: Result<TilePos, usize>,
        hovered: bool,
        text_tool: TextToolId,
    ) -> ControlFlow<()> {
        enum ColorMode {
            Fragment,
            Depth,
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

        let depth = match pos {
            Ok(p) => p.depth(),
            Err(d) => d,
        };

        let should_break = if hovered {
            depth == self.frac_cam.hover_depth()
        } else {
            depth >= self.frac_cam.max_depth()
                || tile_fill.is_leaf() && depth >= self.frac_cam.min_depth()
        };

        let control_flow = if should_break {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        };

        if !DRAW_BRANCHES && control_flow == ControlFlow::Continue(()) {
            return control_flow;
        }

        // FUTURE: add a cursor follower that visually shows the expansion threshold by size
        // maybe solve this once you do bevy tbh
        // shift+scroll zooms the mouse cursor, scroll zooms the camera *and* the cursor

        // TODO: fragment coloring should first try to use the fragment sprite,
        // otherwise use a hash color
        // these should be specified by the fractal itself
        let color_mode = match tile_fill {
            TileFill::Empty => Greyscale,
            TileFill::Partial => Depth,
            TileFill::Full { .. } => {
                if control_flow == ControlFlow::Break(()) {
                    Fragment
                } else {
                    Depth
                }
            }
        };

        let color = match color_mode {
            Depth => {
                const PALETTE: &[Color] = &[RED, ORANGE, GOLD, GREEN, BLUE, PURPLE];
                average(BLACK, PALETTE[pos.map_or(0, |p| p.depth() % PALETTE.len())])
            }
            Fragment => {
                // TODO: have a tile palette based on fragments
                const PALETTE: &[Color] = &[RED, ORANGE, GOLD, GREEN, BLUE, PURPLE];
                PALETTE[id % PALETTE.len()]
            }
            Greyscale => {
                // const PALETTE: &[Color] = &[DARKGRAY, GRAY, LIGHTGRAY];
                // PALETTE[pos.depth() % PALETTE.len()]
                DARKGRAY
            }
        };

        ctx.apply(upscale(self.view_state.scaling()), |ctx| {
            let is_active = pos.is_ok_and(|p| fractory.activated.contains(p));
            // FIXME: 2 of the same tile transformed differently will draw borders wrong.
            if hovered || is_active {
                let border_color = if is_active {
                    if !cache
                        .biome
                        .behavior(cache.fragments.behaviors(), id)
                        .is_empty()
                    {
                        WHITE
                    } else {
                        RED
                    }
                } else {
                    GRAY
                };
                ctx.queue_polygon(&TRIANGLE, border_color);
                ctx.apply(upscale(0.8), |ctx| {
                    ctx.queue_polygon(&TRIANGLE, color);
                });
                if hovered {
                    ctx.apply(
                        shift(0.0, -0.625) * downscale(8.0) * rotate_cw(TAU / 4.0),
                        |ctx| {
                            ctx.queue_polygon(&TRIANGLE, border_color);
                        },
                    )
                }
            } else {
                ctx.queue_polygon(&TRIANGLE, color);
            }
            // ctx.apply(shift(0.0, -0.2) * downscale(4.0), |_| {
            //     let text = format!("{pos:#?}");
            //     text_tool(&text);
            // });
            let name = match names.get(id) {
                Some(name) => name.to_owned(),
                None => id.to_string(),
            };
            let scale = 0.5 / name.len() as f32 + 0.5;
            ctx.apply(upscale(scale), |ctx| ctx.queue_text(text_tool, name));
        });
        control_flow
    }

    fn draw_subtree(
        &self,
        ctx: &mut Context,
        fractory: &Fractory,
        cache: &FractoryCache,
        names: &[String],
        cur_orient: Transform,
        tile: Tile,
        pos: Result<TilePos, usize>,
        text_tool: TextToolId,
    ) {
        let mouse = ctx.mouse_pos().unwrap_or(Vec2::ZERO);
        let hovered = in_triangle(mouse);
        let SlotInfo {
            quad,
            fill,
            symmetries: _,
        } = fractory.fractal.library[tile.id];

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
        .map(|t| downscale(2.0) * t);

        let tile_matrix = transform_to_mat4(tile.orient.into());

        ctx.apply(tile_matrix, |ctx| {
            if !ctx.is_onscreen(&TRIANGLE) {
                return;
            }
            match self.draw_leaf(
                ctx, fractory, cache, names, tile.id, fill, pos, hovered, text_tool,
            ) {
                ControlFlow::Continue(()) => {}
                ControlFlow::Break(()) => return,
            }
            for ((transform, child), subtile) in
                transforms.into_iter().zip(quad.0).zip(SubTile::QUAD.0)
            {
                let orient = cur_orient - tile.orient.transform();

                let pos = match pos {
                    Ok(mut pos) => {
                        pos.push_back(subtile - orient);
                        (pos.depth <= 30).then_some(pos).ok_or(pos.depth as usize)
                    }
                    Err(d) => Err(d + 1),
                };
                ctx.apply(transform, |ctx| {
                    self.draw_subtree(ctx, fractory, cache, names, orient, child, pos, text_tool);
                });
            }
        });
    }

    fn draw(
        &mut self,
        ctx: &mut Context,
        res: &mut Resources,
        fractory_meta: &FractoryMeta,
        cache: &FractoryCache,
        text_tool: TextToolId,
    ) {
        ctx.apply(self.frac_cam.camera, |ctx| {
            // ctx.apply(shift(1.0, 0.0), |ctx| {
            //     ctx.queue_polygon(&TRIANGLE, DARKGRAY)
            // });
            // ctx.apply(shift(-1.0, 0.0), |ctx| ctx.queue_polygon(&TRIANGLE, GRAY));

            // ctx.flush();
            // return;
            self.draw_subtree(
                ctx,
                &fractory_meta.fractory,
                cache,
                cache.fragments.names(),
                Transform::KU,
                fractory_meta.fractory.fractal.root,
                Ok(TilePos::UNIT),
                text_tool,
            );
        });
        ctx.apply(shift(0.0, -0.7) * downscale(5.0), |ctx| {
            let FractalCam {
                camera,
                mouse_depth,
                min_bg_depth,
                ..
            } = self.frac_cam;
            ctx.queue_text(
                text_tool,
                format!(
                    "Selection Depth: 2^{:.2}\n\
                    Background Depth: 2^{:.2}\n\
                    Zoom: 2^{:.2}",
                    mouse_depth,
                    min_bg_depth,
                    self.frac_cam.scale(),
                ),
            );
        });
        ctx.flush();
    }

    fn subtree_click_pos(&mut self, click: Vec2, depth: usize) -> Option<TilePos> {
        if depth > self.frac_cam.hover_depth() {
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
        .map(|t| downscale(2.0) * t)
        .map(|t| t.inverse());

        for (transform, subtile) in transforms.into_iter().zip(SubTile::QUAD.0) {
            let hit_pos = self.subtree_click_pos(
                transform.transform_point3(click.extend(0.0)).truncate(),
                depth + 1,
            );
            if let Some(mut tile_pos) = hit_pos {
                tile_pos.push_front(subtile);
                return Some(tile_pos);
            }
        }
        Some(TilePos::UNIT)
    }

    fn tree_click_pos(&mut self, ctx: &mut Context, click: Click) -> Option<TilePos> {
        if click.held {
            return None;
        }

        let pos = self
            .frac_cam
            .camera
            .inverse()
            .transform_point3(click.pos.extend(0.0))
            .truncate();

        if !in_triangle(pos) {
            return None;
        }

        self.subtree_click_pos(pos, 0)
    }

    fn input_edit(&mut self, hit_pos: TilePos, fractal: &mut Fractal, biome: &Biome) {
        let increment = if is_mouse_button_released(MouseButton::Left) {
            1
        } else if is_mouse_button_released(MouseButton::Right) {
            biome.leaf_count() - 1
        } else {
            debug_assert!(false, "unreachable");
            return;
        };

        let mut tile = fractal.get(hit_pos);
        tile.id += increment;
        tile.id %= biome.leaf_count();
        tile.orient = fractal.library[tile.id].symmetries.into();
        fractal.set(hit_pos, tile);
    }

    fn input_flip(&mut self, hit_pos: TilePos, fractal: &mut Fractal) {
        let tile = fractal.get(hit_pos);
        fractal.set(hit_pos, tile + Transform::FU);
    }

    fn input_act(&mut self, hit_pos: TilePos, activated: &mut ActiveTiles) {
        activated.toggle(hit_pos);
    }

    fn input_rot(&mut self, hit_pos: TilePos, fractal: &mut Fractal) {
        let tf = if is_mouse_button_released(MouseButton::Left) {
            Transform::KL
        } else if is_mouse_button_released(MouseButton::Right) {
            Transform::KR
        } else {
            debug_assert!(false, "unreachable");
            return;
        };

        let mut tile = fractal.get(hit_pos);
        tile += tf;
        fractal.set(hit_pos, tile);
    }

    fn input(
        &mut self,
        ctx: &mut Context,
        res: &mut Resources,
        fractory: &mut Fractory,
        cache: &FractoryCache,
    ) {
        self.frac_cam = (FractalCam::input(ctx) * self.frac_cam).clamp_depth();

        if is_key_pressed(KeyCode::Apostrophe) {
            // dbg!(&fractory.fractal.library);
            fractory.rot_cw();
        }

        if is_key_pressed(KeyCode::Enter) {
            fractory.tick(&cache.fragments.behaviors(), cache.biome.fragment_filter())
        }

        if is_key_pressed(KeyCode::Tab) {
            self.view_state.cycle();
        }

        let shift = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
        let ctrl = is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl);
        'click: {
            let click = if is_mouse_button_released(MouseButton::Left) {
                ctx.get_lmb()
            } else if is_mouse_button_released(MouseButton::Right) {
                ctx.get_rmb()
            } else {
                None
            };
            let Some(hit_pos) = click.and_then(|click| self.tree_click_pos(ctx, click)) else {
                break 'click;
            };

            match (ctrl, shift) {
                (true, true) => self.input_edit(hit_pos, &mut fractory.fractal, &cache.biome),
                (true, false) => {
                    if is_mouse_button_released(MouseButton::Left) {
                        self.input_act(hit_pos, &mut fractory.activated);
                    } else if is_mouse_button_released(MouseButton::Right) {
                        self.input_flip(hit_pos, &mut fractory.fractal);
                    }
                }
                (false, true) => self.input_rot(hit_pos, &mut fractory.fractal),
                _ => {}
            }
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

fn transform_to_mat4(transform: Transform) -> Mat4 {
    let mut matrix = Mat4::IDENTITY;
    if transform.reflected() {
        matrix = flip_x() * matrix;
    }
    match transform.rotation() {
        Rotation::U => {}
        Rotation::R => matrix = rotate_cw(TAU / 3.0) * matrix,
        Rotation::L => matrix = rotate_cc(TAU / 3.0) * matrix,
    }
    matrix
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
