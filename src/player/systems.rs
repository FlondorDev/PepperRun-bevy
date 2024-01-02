use bevy::prelude::*;

use crate::components::{Gravity, Player, Collider};

const PLAYER_SPEED: f32 = 500.;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((SpriteBundle {
        texture: asset_server.load("Player.png"),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    },Gravity, Player, Collider));
    commands.spawn((SpriteBundle {
        texture: asset_server.load("Cobble.png"),
        transform: Transform::from_xyz(50., 0., 0.),
        ..default()
    }, Collider));
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Sprite), (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let (mut player_transform, mut sprite) = player.single_mut();
    let mut direction = Vec2 { x: 0., y: 0. };

    if keyboard_input.pressed(KeyCode::Left) {
        direction.x -= 1.0;
        sprite.flip_x = true;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction.x += 1.0;
        sprite.flip_x = false;
    }
    player_transform.translation.x = player_transform.translation.x + direction.x * PLAYER_SPEED * time.delta_seconds();

    if keyboard_input.pressed(KeyCode::Up) {
        direction.y += 1.0;
        // sprite.flip_x = true;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        direction.y -= 1.0;
        // sprite.flip_x = false;
    }
    player_transform.translation.y = player_transform.translation.y + direction.y * PLAYER_SPEED * time.delta_seconds();
}
