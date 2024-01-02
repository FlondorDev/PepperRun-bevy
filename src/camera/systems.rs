use bevy::prelude::*;

use crate::components::Player;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn update_camera(
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera: Query<&mut Transform, With<Camera2d>>,
) {
    camera.single_mut().translation = player.single().translation.clone();
}
