use crate::ctx::Context;
use ergoquad_2d::prelude::*;
use fractory_common::sim::logic::{
    fractal::{Fractal, SlotInfo, TileFill},
    path::TilePos,
    tile::{Quad, Tile},
};

enum DrawState {
    Leaf,
    Branch(Quad<usize>),
}

struct NodeElement {
    tile: Tile,
    draw_state: DrawState,
}

impl NodeElement {
    fn new(tile: Tile) -> Self {
        Self {
            tile,
            draw_state: DrawState::Leaf,
        }
    }

    fn draw(&self, ctx: &mut Context) {}
}

pub struct FractalElement {
    mouse_hovering: Option<TilePos>,
    nodes: Vec<NodeElement>,
    holes: Vec<usize>,
}

impl FractalElement {
    pub fn new(fractal: &Fractal) -> Self {
        Self {
            mouse_hovering: None,
            nodes: vec![NodeElement::new(fractal.root)],
            holes: vec![],
        }
    }

    pub fn update(&mut self, ctx: &mut Context) {}

    pub fn draw(&self, ctx: &mut Context) {
        for node in &self.nodes {
            node.draw(ctx);
        }
    }
}

fn average(a: Color, b: Color) -> Color {
    Color {
        r: a.r + b.r / 2.0,
        g: a.g + b.g / 2.0,
        b: a.b + b.b / 2.0,
        a: a.a + b.a / 2.0,
    }
}

fn tile_color(fractal: &Fractal, tile_id: usize) -> Color {
    enum ColorMode {
        Fragment,
        Id,
        Greyscale,
    }
    use ColorMode::*;

    let SlotInfo {
        quad,
        fill,
        symmetries: _,
    } = fractal.library[tile_id];

    // TODO: fragment coloring should first try to use the fragment sprite,
    // otherwise use a hash color
    // these should be specified by the fractal itself
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
            // TODO: have a tile palette based on fragments
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

fn tile_name(names: &[String], tile_id: usize) -> String {
    match names.get(tile_id) {
        Some(name) => name.to_owned(),
        None => tile_id.to_string(),
    }
}
