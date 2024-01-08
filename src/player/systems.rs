use bevy::{
    math::vec2,
    prelude::*,
    sprite::{collide_aabb::Collision, Mesh2dHandle},
};

use crate::{
    components::{Collider, CurrentLevel, Gravity, Player, PositionToVec2, Wall},
    level::utils::position_to_world,
    physics::utils::{collide_x, collide_y},
};

const PLAYER_SPEED: f32 = 500.;
const PEPPER_SPEED_MULTIPLIER: f32 = 2.;
const PEPPER_JUMP_FORCE: f32 = 1000.;

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<
        (&Mesh2dHandle, &mut Collider, &mut Player, &mut Transform),
        (With<Player>, Without<Camera2d>),
    >,
    level: Res<CurrentLevel>,
    mut assets_mesh: ResMut<Assets<Mesh>>,
) {
    let player_res = player_query.get_single_mut();
    if player_res.is_ok() {
        let (mesh, mut collider, mut player, mut transform) = player_res.unwrap();
        let mesh = assets_mesh.get_mut(mesh.0.id()).unwrap();
        let mut direction = Vec2 { x: 0., y: 0. };

        if keyboard_input.pressed(KeyCode::A) {
            direction.x -= 1.0;
            mesh.flip_uv(true);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction.x += 1.0;
            mesh.flip_uv(false);
        }

        collider.velocity.x = direction.x * (PLAYER_SPEED * player.speed_mult);

        if (keyboard_input.just_pressed(KeyCode::Space) || keyboard_input.just_pressed(KeyCode::W))
            && player.jumps > 0
        {
            collider.velocity.y = 0.;
            direction.y += 1.0;
            player.jumps -= 1;
        }

        collider.velocity.y = collider.velocity.y + (direction.y * PEPPER_JUMP_FORCE);

        if keyboard_input.pressed(KeyCode::R) {
            let level = level.1.as_ref().unwrap();
            let org_pos = position_to_world(level.player.as_vec2(), Vec2::ONE);
            transform.translation = Vec3::new(org_pos.x, org_pos.y, 0.);
        }
    }
}

pub fn player_wall_collision(
    mut player_query: Query<
        (&mut Transform, &Mesh2dHandle, &mut Collider, &mut Player),
        (With<Player>, Without<Wall>),
    >,
    mut wall_query: Query<
        (&mut Transform, &Mesh2dHandle, &mut Collider),
        (With<Wall>, Without<Player>),
    >,
    assets_mesh: Res<Assets<Mesh>>,
    time: Res<Time>,
) {
    let mut is_grounded = false;
    let player_res = player_query.get_single_mut();
    if player_res.is_ok() {
        let (mut player_transform, player_mesh, mut player_collider, mut player) =
            player_res.unwrap();
        for (mut wall_tranform, wall_mesh, wall_collider) in &mut wall_query {
            let collision_y_opt = collide_y(
                &mut player_transform,
                player_mesh,
                &player_collider,
                &mut wall_tranform,
                wall_mesh,
                &wall_collider,
                &assets_mesh,
                &time,
            );

            if collision_y_opt.is_some() {
                let collision_y = collision_y_opt.unwrap();
                match collision_y.0 {
                    Collision::Bottom => {
                        player_transform.translation.y += collision_y.1;
                        player_collider.velocity.y = wall_collider.velocity.y;
                    }
                    _ => {
                        player_transform.translation.y -= collision_y.1;
                        player.jumps = 2;
                        player_collider.velocity.y = wall_collider.velocity.y;
                        is_grounded = true;
                    }
                }
            }

            let collision_x_opt = collide_x(
                &mut player_transform,
                player_mesh,
                &player_collider,
                &mut wall_tranform,
                wall_mesh,
                &wall_collider,
                &assets_mesh,
                &time,
            );

            if collision_x_opt.is_some() {
                let collision_x = collision_x_opt.unwrap();
                match collision_x.0 {
                    Collision::Right => {
                        player_transform.translation.x -= collision_x.1;
                        player_collider.velocity.x = 0.;
                    }
                    _ => {
                        player_transform.translation.x += collision_x.1;
                        player_collider.velocity.x = 0.;
                    }
                }
            }
        }
        player_collider.is_grounded = is_grounded;
    }
}
