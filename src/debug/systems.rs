use crate::level::utils::spawn_object;
use bevy::prelude::*;

use bevy::render::primitives::Aabb;
use bevy::sprite::Mesh2dHandle;
use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::{MouseScrollUnit, MouseWheel},
};
use crate::structs::components::{Collider, DebugUI, Level, Name, Player, UiEntityRef};
use crate::structs::resources::{CurrentLevel, SelectedUiEntity, SelectedUiMode};
use crate::structs::states::DebugState;
use crate::structs::{ObjectSchema, PositionToVec2, SetSize, Vec2Ser};

#[derive(Component, Default)]
pub struct ScrollingList {
    position: f32,
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn clear_ui(
    mut commands: Commands,
    query: Query<Entity, With<DebugUI>>,
    interaction_query: Query<&UiEntityRef, With<Button>>,
    materials_query: Query<&mut Handle<ColorMaterial>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut selected_ui_entity: ResMut<SelectedUiEntity>,
) {
    interaction_query.iter().for_each(|ui_entity_ref| {
        let handle_material: &Handle<ColorMaterial> = materials_query.get(ui_entity_ref.0).unwrap();
        let material = materials.get_mut(handle_material).unwrap();
        material.color = Color::WHITE;
    });

    selected_ui_entity.0 = None;
    selected_ui_entity.1 = None;

    query
        .iter()
        .for_each(|e| commands.entity(e).despawn_recursive());
}

pub fn setup_debug(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("Roboto-Black.ttf");
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            DebugUI(DebugState::Editor),
        ))
        .with_children(|parent| {
            parent.spawn((
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
                ]),
                Name("X".into()),
            ));
            parent.spawn((
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
                ]),
                Name("Y".into()),
            ));

            parent.spawn((
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
                ]),
                Name("is_grounded".into()),
            ));

            parent.spawn((
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
                ]),
                Name("velx".into()),
            ));

            parent.spawn((
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
                ]),
                Name("vely".into()),
            ));

            parent.spawn((
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
                ]),
                Name("jumps".into()),
            ));

            parent.spawn((
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
                ]),
                Name("speedmult".into()),
            ));
        });
}

pub fn setup_editor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_level: Res<CurrentLevel>,
) {
    let level_name = current_level.0.as_ref().unwrap();
    // root node
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
            DebugUI(DebugState::Editor),
        ))
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
                                height: Val::Percent(100.),
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
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
                                    margin: UiRect {
                                        left: Val::Px(5.),
                                        right: Val::Px(5.),
                                        top: Val::Px(5.),
                                        bottom: Val::Px(20.),
                                    },
                                    ..default()
                                }),
                                // Because this is a distinct label widget and
                                // not button/list item text, this is necessary
                                // for accessibility to treat the text accordingly.
                                Label,
                            ));

                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.),
                                        height: Val::Percent(100.),
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::Center,
                                        align_content: AlignContent::Center,
                                        ..default()
                                    },
                                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Percent(100.),
                                                display: Display::Flex,
                                                flex_direction: FlexDirection::Row,
                                                justify_content: JustifyContent::SpaceEvenly,
                                                ..default()
                                            },
                                            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                            ..default()
                                        })
                                        .with_children(|parent| {
                                            parent
                                                .spawn(ButtonBundle {
                                                    style: Style {
                                                        width: Val::Px(70.0),
                                                        height: Val::Px(60.0),
                                                        border: UiRect::all(Val::Px(5.0)),
                                                        // horizontally center child text
                                                        justify_content: JustifyContent::Center,
                                                        // vertically center child text
                                                        align_items: AlignItems::Center,
                                                        ..default()
                                                    },
                                                    border_color: BorderColor(Color::BLACK),
                                                    background_color: NORMAL_BUTTON.into(),
                                                    ..default()
                                                })
                                                .with_children(|parent| {
                                                    parent.spawn(TextBundle::from_section(
                                                        "XY",
                                                        TextStyle {
                                                            font: asset_server
                                                                .load("Roboto-Black.ttf"),
                                                            font_size: 40.0,
                                                            color: Color::rgb(0.9, 0.9, 0.9),
                                                        },
                                                    ));
                                                });

                                            parent
                                                .spawn(ButtonBundle {
                                                    style: Style {
                                                        width: Val::Px(70.0),
                                                        height: Val::Px(60.0),
                                                        border: UiRect::all(Val::Px(5.0)),
                                                        // horizontally center child text
                                                        justify_content: JustifyContent::Center,
                                                        // vertically center child text
                                                        align_items: AlignItems::Center,
                                                        ..default()
                                                    },
                                                    border_color: BorderColor(Color::BLACK),
                                                    background_color: NORMAL_BUTTON.into(),
                                                    ..default()
                                                })
                                                .with_children(|parent| {
                                                    parent.spawn(TextBundle::from_section(
                                                        "WH",
                                                        TextStyle {
                                                            font: asset_server
                                                                .load("Roboto-Black.ttf"),
                                                            font_size: 40.0,
                                                            color: Color::rgb(0.9, 0.9, 0.9),
                                                        },
                                                    ));
                                                });
                                        });
                                });
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
                            parent.spawn((
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
                            ));
                        });

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                justify_content: JustifyContent::SpaceEvenly,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(ButtonBundle {
                                    style: Style {
                                        width: Val::Px(60.0),
                                        height: Val::Px(60.0),
                                        border: UiRect::all(Val::Px(5.0)),
                                        // horizontally center child text
                                        justify_content: JustifyContent::Center,
                                        // vertically center child text
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    border_color: BorderColor(Color::BLACK),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "+",
                                        TextStyle {
                                            font: asset_server.load("Roboto-Black.ttf"),
                                            font_size: 40.0,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    ));
                                });
                            parent
                                .spawn(ButtonBundle {
                                    style: Style {
                                        width: Val::Px(60.0),
                                        height: Val::Px(60.0),
                                        border: UiRect::all(Val::Px(5.0)),
                                        // horizontally center child text
                                        justify_content: JustifyContent::Center,
                                        // vertically center child text
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    border_color: BorderColor(Color::BLACK),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "-",
                                        TextStyle {
                                            font: asset_server.load("Roboto-Black.ttf"),
                                            font_size: 40.0,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    ));
                                });
                        });
                });
        });
}

pub fn switch_state(
    mut config_store: ResMut<GizmoConfigStore>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    current_debug_state: Res<State<DebugState>>,
    mut debug_state: ResMut<NextState<DebugState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        match current_debug_state.get() {
            DebugState::Debug => {
                debug_state.set(DebugState::Editor);
            }
            DebugState::Editor => {
                debug_state.set(DebugState::None);
            }
            DebugState::None => {
                debug_state.set(DebugState::Debug);
            }
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyO) {
        // AABB gizmos are normally only drawn on entities with a ShowAabbGizmo component
        // We can change this behaviour in the configuration of AabbGizmoGroup
        config_store.config_mut::<AabbGizmoConfigGroup>().1.draw_all ^= true;
    }
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

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            Option<&UiEntityRef>,
            Entity,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    images: Res<Assets<Image>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut selected_ui_entity: ResMut<SelectedUiEntity>,
    mut selected_ui_mode: ResMut<SelectedUiMode>,
    materials_query: Query<&mut Handle<ColorMaterial>>,
) {
    for (interaction, mut color, mut border_color, children, ui_entity_ref, ent) in
        &mut interaction_query
    {
        let text = text_query.get_mut(children[0]).unwrap();
        *color = PRESSED_BUTTON.into();
        border_color.0 = Color::RED;
        if *interaction == Interaction::Pressed {
            match text.sections[0].value.as_str() {
                "-" => {
                    if let Some(selected_ent) = selected_ui_entity.0 {
                        let entity_command = commands.entity(selected_ent);
                        entity_command.despawn_recursive();
                        let button_entity_command = commands.entity(selected_ui_entity.1.unwrap());
                        button_entity_command.despawn_recursive();
                        selected_ui_entity.0 = None;
                        selected_ui_entity.1 = None;
                    }
                }
                "+" => spawn_object(
                    &mut commands,
                    &mut asset_server,
                    &mut meshes,
                    &images,
                    &mut materials,
                    &ObjectSchema {
                        texture: "Cobble.png".to_string(),
                        position: Vec2Ser { x: 5., y: 3. },
                        size: Vec2Ser { x: 1., y: 1. },
                    },
                ),
                "WH" => {
                    selected_ui_mode.0 = String::from("WH");
                }
                "XY" => {
                    selected_ui_mode.0 = String::from("XY");
                }
                _ => {
                    if ui_entity_ref.is_some() {
                        let handle_material: &Handle<ColorMaterial> =
                            materials_query.get(ui_entity_ref.unwrap().0).unwrap();
                        let material = materials.get_mut(handle_material).unwrap();
                        material.color = Color::BLUE;
                        match selected_ui_entity.0 {
                            Some(sel_ent) if sel_ent == ui_entity_ref.unwrap().0 => {
                                selected_ui_entity.0 = None;
                                selected_ui_entity.1 = None;
                            }
                            None | Some(_) => {
                                selected_ui_entity.0 = Some(ui_entity_ref.unwrap().0);
                                selected_ui_entity.1 = Some(ent);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn reset_button(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            Option<&UiEntityRef>,
        ),
        With<Button>,
    >,
    mut text_query: Query<&mut Text>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    selected_ui_entity: Res<SelectedUiEntity>,
    selected_ui_mode: Res<SelectedUiMode>,
    materials_query: Query<&mut Handle<ColorMaterial>>,
) {
    for (interaction, mut color, mut border_color, children, ui_entity_ref) in
        &mut interaction_query
    {
        let text = text_query.get_mut(children[0]).unwrap();
        *color = PRESSED_BUTTON.into();
        border_color.0 = Color::RED;
        match *interaction {
            Interaction::Hovered => match text.sections[0].value.as_str() {
                "-" | "+" | "WH" | "XY" => match &selected_ui_mode.0 {
                    mode if mode == text.sections[0].value.as_str() => {}
                    _ => {
                        *color = HOVERED_BUTTON.into();
                        border_color.0 = Color::WHITE;
                    }
                },
                _ => match selected_ui_entity.0 {
                    Some(sel_ent) if sel_ent == ui_entity_ref.unwrap().0 => {}
                    _ => {
                        if ui_entity_ref.is_some() {
                            if let Ok(handle_material) = materials_query.get(ui_entity_ref.unwrap().0) {
                                let material = materials.get_mut(handle_material).unwrap();
                                material.color = Color::RED;
                                *color = HOVERED_BUTTON.into();
                                border_color.0 = Color::WHITE;
                            }
                        }
                    }
                },
            },
            Interaction::None => match text.sections[0].value.as_str() {
                "-" | "+" | "WH" | "XY" => match &selected_ui_mode.0 {
                    mode if mode == text.sections[0].value.as_str() => {}
                    _ => {
                        *color = NORMAL_BUTTON.into();
                        border_color.0 = Color::BLACK;
                    }
                },
                _ => {
                    if ui_entity_ref.is_some() {
                        match selected_ui_entity.0 {
                            Some(sel_ent) if sel_ent == ui_entity_ref.unwrap().0 => {}
                            _ => {
                                if let Ok(handle_material) =
                                    materials_query.get(ui_entity_ref.unwrap().0)
                                {
                                    if let Some(material) = materials.get_mut(handle_material) {
                                        material.color = Color::WHITE;
                                        *color = NORMAL_BUTTON.into();
                                        border_color.0 = Color::BLACK;
                                    }
                                }
                            }
                        }
                    }
                }
            },
            _ => {}
        }
    }
}

pub fn update_list(
    mut interaction_query: Query<(Entity, Option<&Children>), With<ScrollingList>>,
    mut commands: Commands,
    level_query: Query<(&Name, Entity), With<Level>>,
    asset_server: Res<AssetServer>,
    mut button_query: Query<&Children, With<Button>>,
    mut text_query: Query<&mut Text>,
) {
    let level_len = level_query.iter().len();

    for (ent, child_opt) in &mut interaction_query {
        let mut child_size = if let Some(child) = child_opt {
            child.len()
        } else {
            0
        };

        let mut entity = commands.entity(ent);

        if child_size > level_len {
            entity.despawn_descendants();
            child_size = 0;
        }

        if level_len > child_size {
            for _ in child_size..level_len {
                entity.with_children(|parent| {
                    // List items
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                border: UiRect::all(Val::Px(2.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            border_color: BorderColor(Color::BLACK),
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "".to_string(),
                                TextStyle {
                                    font: asset_server.load("Roboto-Black.ttf"),
                                    font_size: 20.,
                                    ..default()
                                },
                            ));
                        });
                });
            }
        }

        if child_opt.is_some() {
            for (child, (name, entity)) in child_opt.unwrap().iter().zip(level_query.iter()) {
                let mut button_entity = commands.get_entity(*child).unwrap();
                button_entity.try_insert(UiEntityRef(entity));
                let button = button_query.get_mut(*child).unwrap();
                let mut text = text_query.get_mut(button[0]).unwrap();
                text.sections[0].value = name.0.clone();
            }
        }
    }
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
            if name.0.as_str() == "is_grounded" {
                text.sections[1].value = coll.is_grounded.to_string();
            } else if name.0.as_str() == "velx" {
                text.sections[1].value = coll.velocity.x.to_string();
            } else if name.0.as_str() == "vely" {
                text.sections[1].value = coll.velocity.y.to_string();
            }
        }
        if player_res.is_ok() {
            let player = player_res.unwrap();
            if name.0.as_str() == "jumps" {
                text.sections[1].value = player.jumps.to_string();
            } else if name.0.as_str() == "speedmult" {
                text.sections[1].value = player.speed_mult.to_string();
            }
        }
    }
}

pub fn move_items(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    selected_ui_entity: ResMut<SelectedUiEntity>,
    selected_ui_mode: ResMut<SelectedUiMode>,
    mut assets_mesh: ResMut<Assets<Mesh>>,
    mut query: Query<(&mut Transform, &Mesh2dHandle, &mut Aabb)>,
) {
    if let Some(ent) = selected_ui_entity.0 {
        if let Ok((mut transform, mesh_handle, mut aabb)) = query.get_mut(ent) {
            let mesh = assets_mesh.get_mut(mesh_handle.0.id()).unwrap();
            let mut size = mesh.size();

            if keyboard_input.just_pressed(KeyCode::ArrowUp) {
                if selected_ui_mode.0 == "XY" {
                    transform.translation.y += 64.;
                } else {
                    size.y += 64.;
                    transform.translation.y += 32.;
                    mesh.set_size(size);
                    aabb.set_size(size);
                }
            } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
                if selected_ui_mode.0 == "XY" {
                    transform.translation.y -= 64.;
                } else if size.y > 64. {
                    size.y -= 64.;
                    transform.translation.y -= 32.;
                    mesh.set_size(size);
                    aabb.set_size(size);
                }
            } else if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
                if selected_ui_mode.0 == "XY" {
                    transform.translation.x -= 64.;
                } else if size.x > 64. {
                    size.x -= 64.;
                    transform.translation.x -= 32.;
                    mesh.set_size(size);
                    aabb.set_size(size);
                }
            } else if keyboard_input.just_pressed(KeyCode::ArrowRight) {
                if selected_ui_mode.0 == "XY" {
                    transform.translation.x += 64.;
                } else {
                    size.x += 64.;
                    transform.translation.x += 32.;
                    mesh.set_size(size);
                    aabb.set_size(size);
                }
            }
        }
    }
}
