use std::fs;

use bevy::{prelude::*, render::mesh::VertexAttributeValues, sprite::MaterialMesh2dBundle};

use crate::components::{
    ApplicationState, Collider, CurrentLevel, Gravity, Level, LevelSchema, Name, Player, Wall,
};

use super::utils::{position_to_world, spawn_object};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    images: Res<Assets<Image>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut current_level: ResMut<CurrentLevel>,
    mut app_state: ResMut<NextState<ApplicationState>>,
) {
    let levels = fs::read("assets/levels/level.json").unwrap();

    let level =
        serde_json::from_str::<LevelSchema>(String::from_utf8(levels).unwrap().as_str()).unwrap();

    let position = position_to_world(level.player.as_vec2(), Vec2::ONE);

    commands.spawn((
        SpriteBundle {
            texture: asset_server.get_handle("Player.png").unwrap(),
            transform: Transform::from_xyz(position.x, position.y, 0.),
            ..default()
        },
        Gravity,
        Player {
            speed_mult: 1.,
            jumps: 2,
        },
        Collider {
            is_grounded: false,
            velocity: Vec2::ZERO,
        },
        Level,
        Name("Player".to_string()),
    ));

    level.schema.iter().for_each(move |schema| {
        spawn_object(&mut commands, &asset_server, &mut meshes, &images, &mut materials, schema);
    });

    current_level.0 = Some("level".to_string());
    current_level.1 = Some(level);
    app_state.set(ApplicationState::Game);
}
