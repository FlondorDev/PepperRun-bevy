use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;
use bevy::render::primitives::Aabb;

use crate::structs::components::{Collider, Player, Spike};
use crate::structs::states::PlayerState;

pub fn player_spike_collision(
    mut player_query: Query<
        (&mut Transform, &Aabb, &mut Collider, Entity),
        (With<Player>, Without<Spike>),
    >,
    mut spike_query: Query<(&Aabb, &mut Transform), With<Spike>>,
    mut player_state: ResMut<NextState<PlayerState>>,
) {
    if let Ok((mut player_transform, player_aabb, mut collider, player_entity)) =
        player_query.get_single_mut()
    {
        for (spike_aabb, spike_transform) in &mut spike_query {
            let player_bounds = Aabb2d::new(
                player_transform.translation.truncate(),
                player_aabb.half_extents.truncate(),
            );

            let spike_bounds = Aabb2d::new(
                spike_transform.translation.truncate(),
                spike_aabb.half_extents.truncate(),
            );

            if player_bounds.intersects(&spike_bounds) {
                player_state.set(PlayerState::Dead);
            }
        }
    }
}
