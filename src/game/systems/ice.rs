use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;
use bevy::render::primitives::Aabb;

use crate::game::systems::player::PEPPER_SPEED_MULTIPLIER;
use crate::structs::components::{
    AnimationIndices, AnimationTimer, Ice, Level, Oscillante, Pepper, Player,
};
use crate::structs::resources::{Score, UiBarScore};
use crate::structs::states::{ApplicationState, PlayerState};

pub fn player_ice_collision(
    mut player_query: Query<(&Transform, &Aabb), (With<Player>, Without<Ice>)>,
    mut ice_query: Query<(Entity, &Aabb, &mut Transform), With<Ice>>,
    mut score: ResMut<Score>,
    mut ui_score: ResMut<UiBarScore>,
    mut commands: Commands,
) {
    let player_res = player_query.get_single_mut();
    if player_res.is_ok() {
        let (player_transform, player_aabb) = player_res.unwrap();
        for (ice_entity, ice_aabb, ice_transform) in &mut ice_query {
            let player_bounds = Aabb2d::new(
                player_transform.translation.truncate(),
                player_aabb.half_extents.truncate(),
            );

            let ice_bounds = Aabb2d::new(
                ice_transform.translation.truncate(),
                ice_aabb.half_extents.truncate(),
            );

            if player_bounds.intersects(&ice_bounds) {
                commands.entity(ice_entity).despawn();
                score.0 += 50.;
                ui_score.0 += 20.;
            }
        }
    }
}
