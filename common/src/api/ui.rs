//! TODO: traits for how elements should render idk

pub trait Tick {
    fn tick(&mut self);
}

pub trait Context: Default + Tick {}

pub trait Draw {
    type Context: Context;

    fn draw(&self, ctx: Self::Context);
}
