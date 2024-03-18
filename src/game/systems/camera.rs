use bevy::prelude::*;

use crate::structs::components::Player;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 300. ,0.),
        ..default()
    });
}

pub fn update_camera(
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    time: Res<Time>,
) {
    let camera_res = camera_query.get_single_mut();
    let player_res = player_query.get_single();
    if let (Ok(mut camera), Ok(player)) = (camera_res, player_res) {
        camera.translation.x = camera
            .translation.x
            .lerp(player.translation.x, time.delta_seconds() * 4.);
    }
}
