use bevy::render::primitives::Aabb;
use bevy::{prelude::*, sprite::Mesh2dHandle};

use crate::components::Collision;
use crate::components::{Collider, Player, PositionToVec2, Wall};
use crate::game::systems::physics::utils::{collide_x, collide_y};

const PLAYER_SPEED: f32 = 500.;
pub const PEPPER_SPEED_MULTIPLIER: f32 = 2.;
const PEPPER_JUMP_FORCE: f32 = 1000.;

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<
        (&Mesh2dHandle, &mut Collider, &mut Player),
        (With<Player>, Without<Camera2d>),
    >,
    mut assets_mesh: ResMut<Assets<Mesh>>,
) {
    if let Ok((mesh, mut collider, mut player)) = player_query.get_single_mut() {
        let mesh = assets_mesh.get_mut(mesh.0.id()).unwrap();
        let mut direction = Vec2 { x: 0., y: 0. };

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
            mesh.flip_uv(true);
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
            mesh.flip_uv(false);
        }

        collider.velocity.x = direction.x * (PLAYER_SPEED * player.speed_mult);

        if (keyboard_input.just_pressed(KeyCode::Space)
            || keyboard_input.just_pressed(KeyCode::KeyW))
            && player.jumps > 0
        {
            collider.velocity.y = 0.;
            direction.y += 1.0;
            player.jumps -= 1;
        }

        collider.velocity.y += direction.y * PEPPER_JUMP_FORCE;
    }
}

pub fn player_wall_collision(
    mut player_query: Query<
        (&mut Transform, &Aabb, &mut Collider, &mut Player),
        (With<Player>, Without<Wall>),
    >,
    mut wall_query: Query<(&mut Transform, &Aabb, &mut Collider), (With<Wall>, Without<Player>)>,
    time: Res<Time>,
) {
    let mut is_grounded = false;
    let player_res = player_query.get_single_mut();
    if player_res.is_ok() {
        let (mut player_transform, player_aabb, mut player_collider, mut player) =
            player_res.unwrap();
        for (mut wall_tranform, wall_aabb, wall_collider) in &mut wall_query {
            let collision_y_opt = collide_y(
                &mut player_transform,
                player_aabb,
                &player_collider,
                &mut wall_tranform,
                wall_aabb,
                &wall_collider,
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
                player_aabb,
                &player_collider,
                &mut wall_tranform,
                wall_aabb,
                &wall_collider,
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
