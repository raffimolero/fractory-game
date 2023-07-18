mod tree;

use crate::tree::QuadTree;
use ::fractory_common::sim::logic::path::TilePos;
use ::std::f32::consts::TAU;
use fractory_common::sim::logic::path::SubTile;

use ::ergoquad_2d::macroquad; // NOTE: ergoquad2d does not provide its own macro
use ::ergoquad_2d::prelude::*;
use ::rand::prelude::*;

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
    let mouse = mouse_position_local();
    // macroquad calculates delta position wrong, because macroquad
    let mouse_delta = -mouse_delta_position();

    // scroll goes up, transforms zoom in
    let (_scroll_x, scroll_y) = mouse_wheel();
    {
        // zoom
        let scroll_sens = 1.0 / 120.0;
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
            rot += delta * sensitivity;
        }
        if is_key_down(E) {
            rot -= delta * sensitivity;
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

    // resource folder
    set_pc_assets_folder("../assets");
    // font
    let font = load_ttf_font("fonts/VarelaRound-Regular.ttf")
        .await
        .expect("rip varela round");

    // initialize tree
    let mut tree = QuadTree::<u8>::default();

    let mut rng = thread_rng();

    // random path
    let mut path = TilePos::UNIT;
    for _ in 0..6 {
        let tile = match rng.gen_range(0..4) {
            0 => SubTile::C,
            1 => SubTile::U,
            2 => SubTile::R,
            3 => SubTile::L,
            _ => unreachable!(),
        };
        path.push(tile);
    }
    // TODO: implement
    // tree.set(path, 7);

    // random tree
    // let tree = QuadTree::<u8>::rand(&mut rng, 6);

    // specific tree
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

        transform = cam_control() * transform;
        apply(transform, || tree.draw(&draw_num(font, BLACK)));

        // end frame
        next_frame().await
    }
}
