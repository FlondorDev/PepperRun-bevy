use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;
use bevy::render::primitives::Aabb;

use crate::components::Player;
use crate::components::{Oscillante, Pepper};
use crate::game::systems::player::PEPPER_SPEED_MULTIPLIER;

pub fn move_pepper(mut pepper_query: Query<(&mut Oscillante, &mut Transform)>, time: Res<Time>) {
    for (mut _oscillante, mut transform) in &mut pepper_query {
        transform.translation.y += (time.elapsed_seconds() * 2.2).sin() * 0.2;
    }
}

pub fn player_pepper_collision(
    mut player_query: Query<(&Transform, &Aabb, &mut Player), (With<Player>, Without<Pepper>)>,
    mut pepper_query: Query<(Entity, &Pepper, &Aabb, &mut Transform)>,
    mut commands: Commands,
) {
    let player_res = player_query.get_single_mut();
    if player_res.is_ok() {
        let (player_transform, player_aabb, mut player) = player_res.unwrap();
        for (pepper_entity, _pepper, pepper_aabb, pepper_transform) in &mut pepper_query {
            let player_bounds = Aabb2d::new(
                player_transform.translation.truncate(),
                player_aabb.half_extents.truncate(),
            );

            let pepper_bounds = Aabb2d::new(
                pepper_transform.translation.truncate(),
                pepper_aabb.half_extents.truncate(),
            );

            if player_bounds.intersects(&pepper_bounds) {
                commands.entity(pepper_entity).despawn();
                player.speed_mult *= PEPPER_SPEED_MULTIPLIER;
            }
        }
    }
}
