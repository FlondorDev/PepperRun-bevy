use bevy::prelude::*;

use crate::components::{Collider, Gravity};

use super::utils::add_velocity;

const GRAVITY_SPEED: f32 = 50.;
const MAX_GRAVITY_SPEED: f32 = 1400.;


pub fn add_gravity(mut query: Query<&mut Collider, With<Gravity>>) {
    for mut collider in query.iter_mut() {
        collider.velocity.y -= GRAVITY_SPEED;
        collider.velocity.y = collider.velocity.y.clamp(-MAX_GRAVITY_SPEED, f32::INFINITY);
    }
}

pub fn move_entity(mut query: Query<(&mut Transform, &mut Collider)>, time: Res<Time>) {
    for (mut transform, mut collider) in &mut query {
        add_velocity(
            &mut transform.translation,
            &mut collider.velocity,
            &time,
        );
    }
}