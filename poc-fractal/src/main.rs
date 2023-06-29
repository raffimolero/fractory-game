use ::std::{
    array,
    f32::consts::TAU,
    fmt::{Debug, Display},
};

use ::ergoquad_2d::prelude::*;
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
        let is_leaf = rng.gen_ratio(1, 1 << depth);
        if is_leaf {
            Self::Leaf(rng.gen())
        } else {
            Self::Branch(array::from_fn(|_| {
                let is_none = rng.gen_ratio(1, 1 << depth);
                (!is_none).then(|| Box::new(Self::rand(rng, depth - 1)))
            }))
        }
    }
}

impl<T> QuadTree<T> {
    fn _draw(&self, draw_leaf: &impl Fn(&T), depth: usize) {
        let palette = [RED, ORANGE, YELLOW, GREEN, BLUE, PURPLE];
        let col = palette[depth % palette.len()];

        // draw base color
        draw_rectangle(0.0, 0.0, 1.0, 1.0, col);

        // draw outline
        // let outline_thickness = 1.0 / 64.0;
        // draw_rectangle_lines(0.0, 0.0, 1.0, 1.0, outline_thickness, BLACK);

        let children = match self {
            QuadTree::Leaf(val) => return draw_leaf(val),
            QuadTree::Branch(children) => children,
        };

        // margin between child trees
        let margin = 1.0 / 16.0;

        let scale = upscale(0.5 - margin * 1.5);
        for (y, row) in children.chunks_exact(2).enumerate() {
            let y = y as f32 * (0.5 - margin / 2.0) + margin;
            for (x, node) in row.iter().enumerate() {
                let x = x as f32 * (0.5 - margin / 2.0) + margin;
                if let Some(node) = node {
                    apply(shift(x, y) * scale, || node._draw(draw_leaf, depth + 1));
                }
            }
        }
    }

    fn draw(&self, draw_leaf: &impl Fn(&T)) {
        self._draw(draw_leaf, 0);
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

/// builds a quadtree from braces, values, and dots
/// ```
/// let tree = tree! ({
///     { . , (), (), .  },
///     { (), (), . , () },
///     { },
///     .,
/// });
/// println!("{tree:?}");
/// ````
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

/// returns a Mat4 corresponding to how much the map needs to be moved
fn cam_control(mouse_prev: &mut Vec2) -> Mat4 {
    let [mut x, mut y, mut rot] = [0.0; 3];
    let mut flipped = false;
    let mut zoom = 1.0;

    // // nearly every macroquad function uses f32 instead of f64 because that's what `Mat4`s are made of
    // let time = get_time() as f32;
    // for some reason this uses f32s already
    let delta = get_frame_time();

    // check mouse
    // mouse goes downwards, while transforms go upwards
    let mouse = mouse_position_local();
    let mouse_delta = mouse - *mouse_prev;

    // scroll goes up, transforms zoom in
    let (_scroll_x, scroll_y) = mouse_wheel();
    {
        // zoom
        let scroll_sens = 1.0 / 60.0;
        // println!("{scroll_y}");
        zoom *= (2_f32).powf(scroll_y * scroll_sens);
        x -= mouse.x * (zoom - 1.0);
        y -= mouse.y * (zoom - 1.0);

        // drag controls
        if is_mouse_button_down(MouseButton::Left) {
            x += mouse_delta.x;
            y += mouse_delta.y;
        }
    }

    // check keypresses
    {
        use KeyCode::*;

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

        if is_mouse_button_pressed(MouseButton::Right) {
            flipped ^= true;
        }
    }

    *mouse_prev = mouse;

    Mat4::from_scale_rotation_translation(
        Vec3 {
            x: if flipped { -zoom } else { zoom },
            y: zoom,
            z: 1.0,
        },
        Quat::from_rotation_z(rot),
        Vec3 { x, y, z: 0.0 },
    )
}

// NOTE: ergoquad2d does not provide its own macro
use ergoquad_2d::macroquad;

fn window_conf() -> Conf {
    Conf {
        window_title: "WASD/Drag to move, Scroll to zoom, QE to rotate, RMB to flip.".to_owned(),
        window_width: 512,
        window_height: 512,
        fullscreen: false,
        window_resizable: true,
        ..Default::default()
    }
}

fn draw_num(font: Font, color: Color) -> impl Fn(&u8) {
    move |num| {
        let text = num.to_string();
        let params = TextParams {
            font,
            font_size: 64,
            font_scale: 1.0 / 128.0,
            color,
            ..Default::default()
        };
        let dims = measure_text(&text, Some(font), params.font_size, params.font_scale);
        draw_text_ex(
            &text,
            (1.0 - dims.width) / 2.0,
            (1.0 + dims.height) / 2.0,
            params,
        )
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // camera for canvases
    let cam = &mut Camera2D::default();
    cam.zoom = vec2(1.0, -1.0);
    set_camera(cam);

    // mouse data
    let mut mouse_prev = Vec2::default();

    // resource folder
    set_pc_assets_folder("../assets");
    // font
    let font = load_ttf_font("fonts/VarelaRound-Regular.ttf")
        .await
        .expect("rip varela round");

    // initialize tree
    let mut rng = thread_rng();
    let tree = QuadTree::<u8>::rand(&mut rng, 6);
    // let tree = tree! ({
    //     { . , (), (), .  },
    //     { (), (), . , () },
    //     { },
    //     .,
    // });
    println!("{tree:?}");

    // initialize transform
    let mut transform = Mat4::from_scale_rotation_translation(
        Vec3 {
            x: 2.0,
            y: 2.0,
            z: 1.0,
        },
        Quat::IDENTITY,
        Vec3 {
            x: -1.0,
            y: -1.0,
            z: 0.0,
        },
    );

    // main loop
    loop {
        // Quit on Esc
        if let Some(KeyCode::Escape) = get_last_key_pressed() {
            return;
        }

        transform = cam_control(&mut mouse_prev) * transform;
        apply(transform, || tree.draw(&draw_num(font, BLACK)));

        // end frame
        next_frame().await
    }
}
