use bevy::prelude::*;

use crate::components::Player;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn update_camera(
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    let camera_res = camera_query.get_single_mut();
    let player_res = player_query.get_single();
    if camera_res.is_ok() && player_res.is_ok() {
        camera_res.unwrap().translation = player_res.unwrap().translation;
    }
}
