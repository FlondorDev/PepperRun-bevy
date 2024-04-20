use crate::structs::components::{Level, ScoreText, UiBar};
use crate::structs::resources::{Score, UiBarScore};
use crate::structs::states::PlayerState;
use bevy::asset::AssetServer;
use bevy::hierarchy::BuildChildren;
use bevy::prelude::{
    default, AlignItems, Camera2d, Color, ColorMaterial, Commands, Entity, JustifyContent, Mesh,
    NextState, NodeBundle, PositionType, Query, Rectangle, Res, ResMut, Style, Text, Text2dBundle,
    TextBundle, TextSection, TextStyle, Transform, Val, With,
};
use bevy::sprite::Anchor;
use bevy::ui::{JustifySelf, UiRect};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(50.0),
                    height: Val::Percent(4.0),
                    position_type: PositionType::Absolute,
                    left: Val::Percent(25.),
                    top: Val::Percent(0.),
                    margin: UiRect::top(Val::Px(15.)),
                    ..default()
                },
                background_color: Color::MAROON.into(),
                ..default()
            },
            Level,
        ))
        .with_children(|commands| {
            commands
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(98.0),
                            height: Val::Percent(80.0),
                            position_type: PositionType::Absolute,
                            left: Val::Percent(1.),
                            top: Val::Percent(10.),
                            ..default()
                        },
                        ..default()
                    },
                    Level,
                ))
                .with_children(|commands| {
                    commands.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                left: Val::Percent(0.),
                                top: Val::Percent(0.),
                                ..default()
                            },
                            background_color: Color::WHITE.into(),
                            ..default()
                        },
                        UiBar,
                        Level,
                    ));
                });
        });

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: asset_server.load("Roboto-Black.ttf"),
                    font_size: 40.0,
                    ..default()
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font: asset_server.load("Roboto-Black.ttf"),
                    font_size: 40.0,
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            justify_self: JustifySelf::End,
            margin: UiRect::all(Val::Px(15.)),
            ..default()
        }),
        Level,
        ScoreText,
    ));
}

pub fn update_ui(
    mut query_bar: Query<&mut Style, With<UiBar>>,
    mut query_score: Query<&mut Text, With<ScoreText>>,
    mut ui_score: ResMut<UiBarScore>,
    score: Res<Score>,
    mut player_state: ResMut<NextState<PlayerState>>,
) {
    **ui_score -= 0.3;
    **ui_score = (**ui_score).clamp(0., 100.);
    if let Ok(mut ui_bar) = query_bar.get_single_mut() {
        ui_bar.width = Val::Percent(**ui_score);
    }
    if let Ok(mut score_text) = query_score.get_single_mut() {
        score_text.sections[1].value = ((**score) as u16).to_string();
    }

    if **ui_score == 0. {
        player_state.set(PlayerState::Dead);
    }
}
