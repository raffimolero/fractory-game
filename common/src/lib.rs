use std::collections::HashMap;

use api::ui::{Context, Draw};
use sim::logic::{fractal::Fractal, path::TilePos, tile::Tile};

#[cfg(test)]
mod tests;

pub mod api;
pub mod sim;

pub trait Game {
    type GlobalContext: Context;
    type TileElement: From<Tile> + Draw;
}

pub fn run<T: Game>() -> Result<(), ()> {
    let mut ctx = T::GlobalContext::default();
    let mut tree = Fractal::new_binary();

    const TICK_PER_SEC: f32 = 5.0;

    let mut tiles = HashMap::from([(TilePos::UNIT, T::TileElement::from(tree.get(TilePos::UNIT)))]);

    Ok(())
}
