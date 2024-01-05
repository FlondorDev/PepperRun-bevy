use crate::components::{Name, Player, Collider};
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
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Is Grounded: ",
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
            top: Val::Px(80.),
            left: Val::Auto,
            ..default()
        }),
        Name("is_grounded".into()),
    ));
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Vel X: ",
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
            top: Val::Px(120.),
            left: Val::Auto,
            ..default()
        }),
        Name("velx".into()),
    ));
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Vel Y: ",
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
            top: Val::Px(160.),
            left: Val::Auto,
            ..default()
        }),
        Name("vely".into()),
    ));
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Jumps: ",
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
            top: Val::Px(200.),
            left: Val::Auto,
            ..default()
        }),
        Name("jumps".into()),
    ));
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Speed Mult: ",
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
            top: Val::Px(240.),
            left: Val::Auto,
            ..default()
        }),
        Name("speedmult".into()),
    ));
}

pub fn debug_text(
    player_position: Query<&Transform, With<Player>>,
    player_collider: Query<&Collider, With<Player>>,
    player: Query<&Player>,
    mut debug_texts: Query<(&mut Text, &Name)>,
) {
    for (mut text, name) in debug_texts.iter_mut() {
        let pos_res = player_position.get_single();
        let coll_res = player_collider.get_single();
        let player_res = player.get_single();
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
        if coll_res.is_ok() {
            let coll = coll_res.unwrap();
            match name.0.as_str() {
                "is_grounded" => {
                    text.sections[1].value = coll.is_grounded.to_string();
                },
                _ => {}
            }
            match name.0.as_str() {
                "velx" => {
                    text.sections[1].value = coll.velocity.x.to_string();
                },
                _ => {}
            }
            match name.0.as_str() {
                "vely" => {
                    text.sections[1].value = coll.velocity.y.to_string();
                },
                _ => {}
            }
        }
        if player_res.is_ok() {
            let player = player_res.unwrap();
            match name.0.as_str() {
                "jumps" => {
                    text.sections[1].value = player.jumps.to_string();
                },
                _ => {}
            }
            match name.0.as_str() {
                "speedmult" => {
                    text.sections[1].value = player.speed_mult.to_string();
                },
                _ => {}
            }
        }
    }
}
