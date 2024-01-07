use crate::components::{Collider, CurrentLevel, Name, Player};
use bevy::{
    prelude::*,
    render::texture::{ImageSampler, ImageSamplerBorderColor},
};

// pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let font: Handle<Font> = asset_server.load("Roboto-Black.ttf");
//     commands.spawn((
//         TextBundle::from_sections([
//             TextSection::new(
//                 "X: ",
//                 TextStyle {
//                     font: font.clone(),
//                     font_size: 40.,
//                     color: Color::WHITE,
//                     ..default()
//                 },
//             ),
//             TextSection::from_style(TextStyle {
//                 font: font.clone(),
//                 font_size: 40.,
//                 color: Color::WHITE,
//                 ..default()
//             }),
//         ])
//         .with_style(Style {
//             position_type: PositionType::Absolute,
//             top: Val::Auto,
//             left: Val::Auto,
//             ..default()
//         }),
//         Name("X".into()),
//     ));
//     commands.spawn((
//         TextBundle::from_sections([
//             TextSection::new(
//                 "Y: ",
//                 TextStyle {
//                     font: font.clone(),
//                     font_size: 40.,
//                     color: Color::WHITE,
//                     ..default()
//                 },
//             ),
//             TextSection::from_style(TextStyle {
//                 font: font.clone(),
//                 font_size: 40.,
//                 color: Color::WHITE,
//                 ..default()
//             }),
//         ])
//         .with_style(Style {
//             position_type: PositionType::Absolute,
//             top: Val::Px(40.),
//             left: Val::Auto,
//             ..default()
//         }),
//         Name("Y".into()),
//     ));
//     commands.spawn((
//         TextBundle::from_sections([
//             TextSection::new(
//                 "Is Grounded: ",
//                 TextStyle {
//                     font: font.clone(),
//                     font_size: 40.,
//                     color: Color::WHITE,
//                     ..default()
//                 },
//             ),
//             TextSection::from_style(TextStyle {
//                 font: font.clone(),
//                 font_size: 40.,
//                 color: Color::WHITE,
//                 ..default()
//             }),
//         ])
//         .with_style(Style {
//             position_type: PositionType::Absolute,
//             top: Val::Px(80.),
//             left: Val::Auto,
//             ..default()
//         }),
//         Name("is_grounded".into()),
//     ));
//     commands.spawn((
//         TextBundle::from_sections([
//             TextSection::new(
//                 "Vel X: ",
//                 TextStyle {
//                     font: font.clone(),
//                     font_size: 40.,
//                     color: Color::WHITE,
//                     ..default()
//                 },
//             ),
//             TextSection::from_style(TextStyle {
//                 font: font.clone(),
//                 font_size: 40.,
//                 color: Color::WHITE,
//                 ..default()
//             }),
//         ])
//         .with_style(Style {
//             position_type: PositionType::Absolute,
//             top: Val::Px(120.),
//             left: Val::Auto,
//             ..default()
//         }),
//         Name("velx".into()),
//     ));
//     commands.spawn((
//         TextBundle::from_sections([
//             TextSection::new(
//                 "Vel Y: ",
//                 TextStyle {
//                     font: font.clone(),
//                     font_size: 40.,
//                     color: Color::WHITE,
//                     ..default()
//                 },
//             ),
//             TextSection::from_style(TextStyle {
//                 font: font.clone(),
//                 font_size: 40.,
//                 color: Color::WHITE,
//                 ..default()
//             }),
//         ])
//         .with_style(Style {
//             position_type: PositionType::Absolute,
//             top: Val::Px(160.),
//             left: Val::Auto,
//             ..default()
//         }),
//         Name("vely".into()),
//     ));
//     commands.spawn((
//         TextBundle::from_sections([
//             TextSection::new(
//                 "Jumps: ",
//                 TextStyle {
//                     font: font.clone(),
//                     font_size: 40.,
//                     color: Color::WHITE,
//                     ..default()
//                 },
//             ),
//             TextSection::from_style(TextStyle {
//                 font: font.clone(),
//                 font_size: 40.,
//                 color: Color::WHITE,
//                 ..default()
//             }),
//         ])
//         .with_style(Style {
//             position_type: PositionType::Absolute,
//             top: Val::Px(200.),
//             left: Val::Auto,
//             ..default()
//         }),
//         Name("jumps".into()),
//     ));
//     commands.spawn((
//         TextBundle::from_sections([
//             TextSection::new(
//                 "Speed Mult: ",
//                 TextStyle {
//                     font: font.clone(),
//                     font_size: 40.,
//                     color: Color::WHITE,
//                     ..default()
//                 },
//             ),
//             TextSection::from_style(TextStyle {
//                 font: font.clone(),
//                 font_size: 40.,
//                 color: Color::WHITE,
//                 ..default()
//             }),
//         ])
//         .with_style(Style {
//             position_type: PositionType::Absolute,
//             top: Val::Px(240.),
//             left: Val::Auto,
//             ..default()
//         }),
//         Name("speedmult".into()),
//     ));
// }

// pub fn debug_text(
//     player_position: Query<&Transform, With<Player>>,
//     player_collider: Query<&Collider, With<Player>>,
//     player: Query<&Player>,
//     mut debug_texts: Query<(&mut Text, &Name)>,
// ) {
//     for (mut text, name) in debug_texts.iter_mut() {
//         let pos_res = player_position.get_single();
//         let coll_res = player_collider.get_single();
//         let player_res = player.get_single();
//         if pos_res.is_ok() {
//             let pos = pos_res.unwrap();
//             match name.0.as_str() {
//                 "X" => {
//                     text.sections[1].value = pos.translation.x.to_string();
//                 }
//                 "Y" => {
//                     text.sections[1].value = pos.translation.y.to_string();
//                 }
//                 _ => {}
//             }
//         }
//         if coll_res.is_ok() {
//             let coll = coll_res.unwrap();
//             match name.0.as_str() {
//                 "is_grounded" => {
//                     text.sections[1].value = coll.is_grounded.to_string();
//                 }
//                 _ => {}
//             }
//             match name.0.as_str() {
//                 "velx" => {
//                     text.sections[1].value = coll.velocity.x.to_string();
//                 }
//                 _ => {}
//             }
//             match name.0.as_str() {
//                 "vely" => {
//                     text.sections[1].value = coll.velocity.y.to_string();
//                 }
//                 _ => {}
//             }
//         }
//         if player_res.is_ok() {
//             let player = player_res.unwrap();
//             match name.0.as_str() {
//                 "jumps" => {
//                     text.sections[1].value = player.jumps.to_string();
//                 }
//                 _ => {}
//             }
//             match name.0.as_str() {
//                 "speedmult" => {
//                     text.sections[1].value = player.speed_mult.to_string();
//                 }
//                 _ => {}
//             }
//         }
//     }
// }

use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut current_level: ResMut<CurrentLevel>,
) {
    let level_name = current_level.0.as_ref().unwrap();
    let level = current_level.1.as_ref().unwrap();
    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(200.),
                        border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                ..default()
                            },
                            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn((
                                TextBundle::from_section(
                                    format!("Livello: {}", level_name),
                                    TextStyle {
                                        font: asset_server.load("Roboto-Black.ttf"),
                                        font_size: 30.0,
                                        ..default()
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Px(5.)),
                                    ..default()
                                }),
                                // Because this is a distinct label widget and
                                // not button/list item text, this is necessary
                                // for accessibility to treat the text accordingly.
                                Label,
                            ));
                        });
                });
            // right vertical fill
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(200.),
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        TextBundle::from_section(
                            "Object List",
                            TextStyle {
                                font: asset_server.load("Roboto-Black.ttf"),
                                font_size: 25.,
                                ..default()
                            },
                        ),
                        Label,
                    ));
                    // List with hidden overflow
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_self: AlignSelf::Stretch,
                                height: Val::Percent(50.),
                                overflow: Overflow::clip_y(),
                                ..default()
                            },
                            background_color: Color::rgb(0.10, 0.10, 0.10).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Moving panel
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::Column,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    ScrollingList::default(),
                                    AccessibilityNode(NodeBuilder::new(Role::List)),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        TextBundle::from_section(
                                            "Player",
                                            TextStyle {
                                                font: asset_server.load("Roboto-Black.ttf"),
                                                font_size: 20.,
                                                ..default()
                                            },
                                        ),
                                        Label,
                                        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                    ));
                                    // List items
                                    for i in &level.schema {
                                        parent.spawn((
                                            TextBundle::from_section(
                                                i.texture.split_once(".").unwrap().0,
                                                TextStyle {
                                                    font: asset_server.load("Roboto-Black.ttf"),
                                                    font_size: 20.,
                                                    ..default()
                                                },
                                            ),
                                            Label,
                                            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                        ));
                                    }
                                });
                        });
                });
        });
}

#[derive(Component, Default)]
pub struct ScrollingList {
    position: f32,
}

pub fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.top = Val::Px(scrolling_list.position);
        }
    }
}
