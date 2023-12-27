use bevy::{prelude::*, text::Text2dBounds};

pub fn text(value: String, font_size: f32, mut bounds: Vec2) -> Text2dBundle {
    let size = 0.5 / font_size / (value.len() as f32).sqrt();
    bounds /= size * 2.0;
    Text2dBundle {
        text: Text {
            sections: vec![TextSection {
                value,
                style: TextStyle {
                    font: default(), // TODO: load a copyright free font
                    font_size,
                    color: Color::WHITE,
                },
            }],
            alignment: TextAlignment::Center,
            linebreak_behavior: bevy::text::BreakLineOn::WordBoundary,
        },
        text_2d_bounds: Text2dBounds { size: bounds },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.5),
            rotation: default(),
            scale: Vec2::splat(size).extend(1.0),
        },
        ..default()
    }
}
