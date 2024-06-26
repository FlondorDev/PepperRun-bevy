use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::render::primitives::Aabb;
use bevy::{prelude::*, sprite::Mesh2dHandle};
use egui_dock::egui::Key::M;

use crate::game::systems::physics::utils::{collide_x, collide_y};
use crate::structs::components::{
    AnimationIndices, AnimationTimer, Collider, Level, Pepper, Player, Spike, Wall,
};
use crate::structs::states::PlayerState;
use crate::structs::Collision;

const PLAYER_SPEED: f32 = 500.;
pub const PEPPER_SPEED_MULTIPLIER: f32 = 2.;
const PEPPER_JUMP_FORCE: f32 = 1000.;

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<
        (&mut Transform, &Mesh2dHandle, &mut Collider, &mut Player),
        (With<Player>, Without<Camera2d>),
    >,
    mut assets_mesh: ResMut<Assets<Mesh>>,
) {
    if let Ok((mut transform, mesh, mut collider, mut player)) = player_query.get_single_mut() {
        let mesh = assets_mesh.get_mut(mesh.0.id()).unwrap();
        let mut direction = Vec2 { x: 0., y: 0. };

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI); //mesh.flip_uv(true);
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
            transform.rotation = Quat::from_rotation_y(0.); //mesh.flip_uv(false);
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

pub fn player_death(
    mut player_query: Query<
        (&mut Transform, &mut Collider, Entity),
        (With<Player>, Without<Spike>),
    >,
    mut commands: Commands,
) {
    if let Ok((mut player_transform, mut collider, player_entity)) = player_query.get_single_mut() {
        commands.entity(player_entity).remove::<Player>();
        commands.entity(player_entity).despawn_descendants();
        collider.velocity.y = 1000.;
        collider.is_grounded = false;
        let rot = player_transform.rotation.to_euler(EulerRot::XYZ);
        player_transform.rotation = Quat::from_euler(
            EulerRot::XYZ,
            rot.0,
            rot.1,
            rot.2 + std::f32::consts::PI * 0.5,
        );

        // TODO: play death sound;
    }
}
