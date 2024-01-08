use std::fs;

use bevy::{prelude::*, render::mesh::VertexAttributeValues, sprite::MaterialMesh2dBundle};

use crate::components::{
    ApplicationState, Collider, CurrentLevel, Gravity, Level, LevelSchema, Name, ObjectSchema,
    Player, Vec2Ser, Wall,
};

use super::utils::{generate_mesh2d, position_to_world, spawn_object};

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

    let mesh = generate_mesh2d(
        &mut commands,
        &asset_server,
        &mut meshes,
        &images,
        &mut materials,
        &ObjectSchema {
            position: Vec2Ser {
                x: position.x,
                y: position.y,
            },
            size: Vec2Ser {
                x: 1.,
                y: 1.,
            },
            texture: "Player.png".to_string()
        },
    );

    commands.spawn((
        mesh,
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
        spawn_object(
            &mut commands,
            &asset_server,
            &mut meshes,
            &images,
            &mut materials,
            schema,
        );
    });

    current_level.0 = Some("level".to_string());
    current_level.1 = Some(level);
    app_state.set(ApplicationState::Game);
}
