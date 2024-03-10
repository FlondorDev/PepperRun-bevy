use bevy::render::primitives::Aabb;
use bevy::{
    ecs::system::Res,
    math::{Vec2, Vec3},
    time::Time,
    transform::components::Transform,
};

use crate::components::{Collider, Collision};

#[inline]
pub fn add_velocity(translation: &mut Vec3, velocity: &Vec2, time: &Res<Time>) {
    add_velocity_x(translation, velocity, time);
    add_velocity_y(translation, velocity, time);
}

#[inline]
pub fn add_velocity_x(translation: &mut Vec3, velocity: &Vec2, time: &Res<Time>) {
    translation.x += velocity.x * time.delta_seconds();
}

#[inline]
pub fn add_velocity_y(translation: &mut Vec3, velocity: &Vec2, time: &Res<Time>) {
    translation.y += velocity.y * time.delta_seconds();
}

fn x(
    org_a_pos: Vec3,
    a_pos: Vec3,
    a_size: Vec2,
    b_pos: Vec3,
    b_size: Vec2,
) -> Option<(Collision, f32)> {
    let org_a_min = org_a_pos.truncate() - a_size / 2.0;
    let org_a_max = org_a_pos.truncate() + a_size / 2.0;

    let a_min = a_pos.truncate() - a_size / 2.0;
    let a_max = a_pos.truncate() + a_size / 2.0;

    let b_min = b_pos.truncate() - b_size / 2.0;
    let b_max = b_pos.truncate() + b_size / 2.0;

    // check to see if the two rectangles are intersecting
    if a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y {
        // check to see if we hit on the left or right side
        let x_collision = if a_min.x < b_min.x && a_max.x > b_min.x && a_max.x < b_max.x {
            (Collision::Left, b_min.x - org_a_max.x)
        } else if a_min.x > b_min.x && a_min.x < b_max.x && a_max.x > b_max.x {
            (Collision::Right, org_a_min.x - b_max.x)
        } else {
            (Collision::Inside, -f32::INFINITY)
        };

        Some(x_collision)
    } else {
        None
    }
}

fn y(
    org_a_pos: Vec3,
    a_pos: Vec3,
    a_size: Vec2,
    b_pos: Vec3,
    b_size: Vec2,
) -> Option<(Collision, f32)> {
    let org_a_min = org_a_pos.truncate() - a_size / 2.0;
    let org_a_max = org_a_pos.truncate() + a_size / 2.0;

    let a_min = a_pos.truncate() - a_size / 2.0;
    let a_max = a_pos.truncate() + a_size / 2.0;

    let b_min = b_pos.truncate() - b_size / 2.0;
    let b_max = b_pos.truncate() + b_size / 2.0;

    // check to see if the two rectangles are intersecting
    if a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y {
        // check to see if we hit on the top or bottom side
        let y_collision = if a_min.y < b_min.y && a_max.y > b_min.y && a_max.y < b_max.y {
            (Collision::Bottom, b_min.y - org_a_max.y)
        } else if a_min.y > b_min.y && a_min.y < b_max.y && a_max.y > b_max.y {
            (Collision::Top, org_a_min.y - b_max.y)
        } else {
            (Collision::Inside, -f32::INFINITY)
        };

        Some(y_collision)
    } else {
        None
    }
}

pub fn collide_y(
    a_pos: &mut Transform,
    a_aabb: &Aabb,
    a_col: &Collider,
    b_pos: &mut Transform,
    b_aabb: &Aabb,
    b_col: &Collider,
    time: &Res<Time>,
) -> Option<(Collision, f32)> {
    let a_size = a_aabb.half_extents.truncate() * 2.;
    let b_size = b_aabb.half_extents.truncate() * 2.;

    let mut new_a_pos_y = a_pos.translation.clone();
    let mut new_b_pos_y = b_pos.translation.clone();
    add_velocity_y(&mut new_a_pos_y, &a_col.velocity, &time);
    add_velocity_y(&mut new_b_pos_y, &b_col.velocity, &time);

    y(a_pos.translation, new_a_pos_y, a_size, new_b_pos_y, b_size)
}

pub fn collide_x(
    a_pos: &mut Transform,
    a_aabb: &Aabb,
    a_col: &Collider,
    b_pos: &mut Transform,
    b_aabb: &Aabb,
    b_col: &Collider,
    time: &Res<Time>,
) -> Option<(Collision, f32)> {
    let a_size = a_aabb.half_extents.truncate() * 2.;
    let b_size = b_aabb.half_extents.truncate() * 2.;

    let mut new_a_pos_x = a_pos.translation.clone();
    let mut new_b_pos_x = b_pos.translation.clone();
    add_velocity_x(&mut new_a_pos_x, &a_col.velocity, &time);
    add_velocity_x(&mut new_b_pos_x, &b_col.velocity, &time);

    x(a_pos.translation, new_a_pos_x, a_size, new_b_pos_x, b_size)
}
