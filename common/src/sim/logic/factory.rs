// // TODO: fractory game logic

// use super::fractal::Fractal;
// use crate::api::ui::Draw;

// pub struct Fractory {
//     fractal: Fractal,
// }

// impl Fractory {
//     fn _draw(&self, tile: Tile, depth: usize) {
//         const PALETTE: &[Color] = &[RED, ORANGE, YELLOW, GREEN, BLUE, PURPLE];
//         let col = PALETTE[depth % PALETTE.len()];

//         let draw_base = || {
//             draw_rectangle(0.0, 0.0, 1.0, 1.0, col);

//             // // draw outline
//             // let outline_thickness = 1.0 / 64.0;
//             // draw_rectangle_lines(0.0, 0.0, 1.0, 1.0, outline_thickness, BLACK);
//         };
//         match self {
//             Node::Free => {}
//             Node::Bad => draw_poly(0.0, 0.0, 4, 1.0, 45.0, col),
//             Node::Leaf(val) => {
//                 draw_base();
//                 draw_leaf(*val);
//             }
//             Node::Branch(children) => {
//                 draw_base();

//                 // margin between child trees
//                 let margin = 1.0 / 16.0;

//                 let scale = upscale(0.5 - margin * 1.5);
//                 for (y, row) in children.0.chunks_exact(2).enumerate() {
//                     let y = y as f32 * (0.5 - margin / 2.0) + margin;
//                     for (x, node) in row.iter().enumerate() {
//                         let x = x as f32 * (0.5 - margin / 2.0) + margin;
//                         apply(shift(x, y) * scale, || node._draw(draw_leaf, depth + 1))
//                     }
//                 }
//             }
//         }
//     }
// }

// impl Draw for Fractory {
//     type Context = ();

//     fn draw(&self, ctx: Self::Context) {}
// }
