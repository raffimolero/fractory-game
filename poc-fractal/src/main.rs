use ::std::{
    f32::consts::TAU,
    fmt::{Debug, Display},
};
use std::array;

use ::ergoquad_2d::prelude::*;
// the macroquad rand module is terrible. why does macroquad do this?
use ::rand::{distributions::Standard, prelude::*};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro() {
        let tre = tree! {
            { 1, 2, 3, { 4, 5, ., 7} }
        };
        println!("{tre}");
    }

    #[test]
    fn test_rand() {
        let mut rng = thread_rng();
        let tre = QuadTree::<u8>::rand(&mut rng, 5);
        println!("{tre}");
    }
}

enum QuadTree<T> {
    Leaf(T),
    Branch([Option<Box<Self>>; 4]),
}

impl<T> QuadTree<T> {
    fn rand(rng: &mut impl Rng, depth: usize) -> Self
    where
        Standard: Distribution<T>,
    {
        // let mut possible_trees: u32 = 1;
        // let mut possible_branches = !0;
        // for _ in 0..depth {
        //     possible_branches = possible_trees + 1;
        //     possible_trees = possible_branches.pow(4) + 1;
        // }

        // let is_leaf = rng.gen_ratio(1, possible_trees);
        let is_leaf = rng.gen_ratio(1, 1 << depth);
        if is_leaf {
            Self::Leaf(rng.gen())
        } else {
            Self::Branch(array::from_fn(|_| {
                // let is_some = !rng.gen_ratio(1, possible_branches);
                let is_some = !rng.gen_ratio(1, 2);
                is_some.then(|| Box::new(Self::rand(rng, depth - 1)))
            }))
        }
    }
}

impl QuadTree<()> {
    fn _draw(&self, depth: usize) {
        let palette = [RED, ORANGE, YELLOW, GREEN, BLUE, PURPLE];
        let col = palette[depth % palette.len()];

        // draw base color
        draw_rectangle(0.0, 0.0, 1.0, 1.0, col);

        // draw outline
        let outline_thickness = 1.0 / 64.0;
        draw_rectangle_lines(0.0, 0.0, 1.0, 1.0, outline_thickness, BLACK);

        let Self::Branch(children) = self else {
            return;
        };

        // margin between child trees
        let margin = 1.0 / 32.0;

        let scale = upscale(0.5 - margin * 2.0);
        for (y, row) in children.chunks_exact(2).enumerate() {
            let y = y as f32 * 0.5 + margin;
            for (x, node) in row.iter().enumerate() {
                let x = x as f32 * 0.5 + margin;
                if let Some(node) = node {
                    apply(shift(x, y) * scale, || node._draw(depth + 1));
                }
            }
        }
    }

    fn draw(&self) {
        self._draw(0);
    }
}

impl<T: Display> Display for QuadTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuadTree::Leaf(val) => val.fmt(f),
            QuadTree::Branch(children) => {
                write!(f, "{{ ")?;
                for child in children {
                    match child {
                        Some(val) => write!(f, "{val}")?,
                        None => write!(f, ".")?,
                    }
                    write!(f, " ")?;
                }
                write!(f, "}}")
            }
        }
    }
}

impl<T: Debug> Debug for QuadTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuadTree::Leaf(val) => val.fmt(f),
            QuadTree::Branch(children) => {
                write!(f, "{{ ")?;
                for child in children {
                    match child {
                        Some(val) => write!(f, "{val:?}")?,
                        None => write!(f, ".")?,
                    }
                    write!(f, " ")?;
                }
                write!(f, "}}")
            }
        }
    }
}

macro_rules! tree {
    (node .) => {
        None
    };
    (node $t:tt) => {
        Some(Box::new(tree!($t)))
    };

    ({ $tl:tt,  $tr:tt, $bl:tt, $br:tt $(,)? }) => {
        QuadTree::Branch([
            tree!(node $tl),
            tree!(node $tr),
            tree!(node $bl),
            tree!(node $br),
        ])
    };
    ($t:expr) => {
        QuadTree::Leaf($t)
    };
}
pub(crate) use tree;

fn window_conf() -> Conf {
    Conf {
        window_title: "WASD/Drag to move, Scroll to zoom, QE to rotate.".to_owned(),
        window_width: 512,
        window_height: 512,
        fullscreen: false,
        window_resizable: true,
        ..Default::default()
    }
}

// NOTE: ergoquad2d does not provide its own macro
use ergoquad_2d::macroquad;
#[macroquad::main(window_conf)]
async fn main() {
    // camera for canvases
    let cam = &mut Camera2D::default();
    cam.zoom = vec2(1.0, -1.0);
    set_camera(cam);

    // mouse data
    let mut mouse = mouse_position_local();
    let mut mouse_prev;

    // resource folder
    set_pc_assets_folder("../assets");
    // font
    let font = load_ttf_font("fonts/VarelaRound-Regular.ttf")
        .await
        .expect("rip varela round");

    // initialize tree
    let mut rng = thread_rng();
    let tree = QuadTree::<()>::rand(&mut rng, 5);
    // let tree = tree! ({
    //     { . , (), (), .  },
    //     { (), (), . , () },
    //     { },
    //     .,
    // });
    println!("{tree:?}");

    // initialize transforms
    let [mut x, mut y, mut rot] = [0.75, 0.75, 0.0];
    let mut flipped = false;
    let mut zoom = 0.25;

    // main loop
    loop {
        // nearly every macroquad function uses f32 instead of f64 because that's what `Mat4`s are made of
        let time = get_time() as f32;
        // for some reason this uses f32s already
        let delta = get_frame_time();

        // check mouse
        // mouse goes downwards, while transforms go upwards
        mouse_prev = mouse;
        mouse = mouse_position_local();
        let mouse_delta = mouse - mouse_prev;

        // scroll goes up, transforms zoom in
        let (_scroll_x, scroll_y) = mouse_wheel();
        {
            // zoom
            let scroll_sens = 1.0 / 120.0;
            // println!("{scroll_y}");
            zoom *= (2_f32).powf(scroll_y * scroll_sens);

            // drag controls
            if is_mouse_button_down(MouseButton::Left) {
                x += mouse_delta.x;
                y += mouse_delta.y;
            }
        }

        // check keypresses
        {
            use KeyCode::*;
            // Quit on Esc
            if let Some(Escape) = get_last_key_pressed() {
                return;
            }

            // WASD movement, y goes down
            if is_key_down(W) {
                y -= delta;
            }
            if is_key_down(S) {
                y += delta;
            }
            if is_key_down(A) {
                x -= delta;
            }
            if is_key_down(D) {
                x += delta;
            }

            // rotation, clockwise
            let sensitivity = TAU / 2.0; // no i will not use pi
            if is_key_down(Q) {
                rot -= delta * sensitivity;
            }
            if is_key_down(E) {
                rot += delta * sensitivity;
            }

            if is_key_pressed(CapsLock) {
                flipped ^= true;
            }
        }

        apply(shift(-1.0, -1.0) * upscale(2.0), || tree.draw());

        // end frame
        next_frame().await
    }
}
