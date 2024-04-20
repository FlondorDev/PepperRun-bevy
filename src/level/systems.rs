use std::fs;

use crate::asset_loader::json_loader::JsonAsset;
use crate::structs::components::{Collider, Gravity, Level, Name, Player};
use crate::structs::resources::{CurrentLevel, Score, UiBarScore};
use crate::structs::states::{ApplicationState, PlayerState};
use crate::structs::{LevelSchema, ObjectSchema, Vec2Ser};
use bevy::prelude::*;

use super::utils::{generate_mesh2d, spawn_object};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    images: Res<Assets<Image>>,
    jsons: Res<Assets<JsonAsset>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut current_level: ResMut<CurrentLevel>,
    mut app_state: ResMut<NextState<ApplicationState>>,
    mut player_state: ResMut<NextState<PlayerState>>,
) {
    let level_json = jsons
        .get(asset_server.get_handle("levels/level.json").unwrap().id())
        .unwrap();

    let level = serde_json::from_value::<LevelSchema>((*level_json).clone()).unwrap();

    let mut mesh = generate_mesh2d(
        &asset_server,
        &mut meshes,
        &images,
        &mut materials,
        &ObjectSchema {
            position: level.player.clone(),
            size: Vec2Ser { x: 1., y: 1. },
            texture: "Player.png".to_string(),
        },
    );

    // hack per player come primo layer
    mesh.transform.translation.z = 3.;

    // commands.spawn((
    //     (SpriteBundle {
    //         texture: asset_server.load(format!("texture/{}", level.bg)),
    //         transform: Transform {
    //             rotation: Quat::from_rotation_z(0.),
    //             scale: Vec3::new(3440. / 64., 1440. / 64., 0.),
    //             translation: Vec3::new(3440. / 2., 0., -1.),
    //         },
    //         ..default()
    //     }),
    //     Level,
    // ));

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


    app_state.set(ApplicationState::Game);
    player_state.set(PlayerState::Normal);
    current_level.0 = Some("level".to_string());
    current_level.1 = Some(level);
}

pub fn despawn_all(
    mut commands: Commands,
    query: Query<Entity, With<Level>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_state: ResMut<NextState<ApplicationState>>,
    mut current_app_state: ResMut<State<ApplicationState>>,
    mut player_state: ResMut<NextState<PlayerState>>,
    mut ui_bar: ResMut<UiBarScore>,
    mut score: ResMut<Score>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        for entity in query.iter() {
            commands.entity(entity).despawn()
        }
        app_state.set(ApplicationState::AssetsLoaded);
        player_state.set(PlayerState::None);
        *ui_bar = UiBarScore::default();
        *score = Score::default();
    }
}
