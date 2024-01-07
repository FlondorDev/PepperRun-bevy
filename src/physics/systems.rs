use bevy::prelude::*;

use crate::components::{Collider, Gravity, Player};

use super::utils::*;

const GRAVITY_SPEED: f32 = 50.;
const MAX_GRAVITY_SPEED: f32 = 1400.;

pub fn add_gravity(mut query: Query<(&mut Collider, Option<&Player>), With<Gravity>>) {
    for (mut collider, player) in query.iter_mut() {
        collider.velocity.y -= GRAVITY_SPEED;
        if player.is_some() {
            collider.velocity.y = collider.velocity.y.clamp(-MAX_GRAVITY_SPEED, f32::INFINITY);
        } else {
            collider.velocity.y = collider.velocity.y.clamp(-MAX_GRAVITY_SPEED, f32::INFINITY);
        }
    }
}

pub fn move_entity(mut query: Query<(&mut Transform, &mut Collider)>, time: Res<Time>) {
    for (mut transform, mut collider) in &mut query {
        if collider.is_grounded {
            add_velocity_x(&mut transform.translation, &mut collider.velocity, &time);
        }else{
            add_velocity(&mut transform.translation, &mut collider.velocity, &time);
        }
    }
}
