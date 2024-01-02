use crate::components::{Name, Player};
use bevy::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("Roboto-Black.ttf");
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "X: ",
                TextStyle {
                    font: font.clone(),
                    font_size: 40.,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font: font.clone(),
                font_size: 40.,
                color: Color::WHITE,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Auto,
            left: Val::Auto,
            ..default()
        }),
        Name("X".into()),
    ));
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Y: ",
                TextStyle {
                    font: font.clone(),
                    font_size: 40.,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font: font.clone(),
                font_size: 40.,
                color: Color::WHITE,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(40.),
            left: Val::Auto,
            ..default()
        }),
        Name("Y".into()),
    ));
}

pub fn debug_text(
    sprite_position: Query<&Transform, With<Player>>,
    mut debug_texts: Query<(&mut Text, &Name)>,
) {
    for (mut text, name) in debug_texts.iter_mut() {
        let pos_res = sprite_position.get_single();
        if pos_res.is_ok() {
            let pos = pos_res.unwrap();
            match name.0.as_str() {
                "X" => {
                    text.sections[1].value = pos.translation.x.to_string();
                }
                "Y" => {
                    text.sections[1].value = pos.translation.y.to_string();
                }
                _ => {}
            }
        }
    }
}
