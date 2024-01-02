use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::components::{Collider, Gravity, Player};
use bevy::sprite::collide_aabb::Collision;

const GRAVITY_SPEED: f32 = 100.;

pub fn add_gravity(mut query: Query<&mut Transform, With<Gravity>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.translation.y -= GRAVITY_SPEED * time.delta_seconds();
    }
}

pub fn check_collision(
    mut env: Query<(&mut Transform, &Handle<Image>), (With<Collider>, Without<Player>)>,
    mut player_query: Query<(&mut Transform, &Handle<Image>), (With<Collider>, With<Player>)>,
    assets: Res<Assets<Image>>,
) {
    let player_res = player_query.get_single_mut();
    if player_res.is_ok() {
        let (mut player, player_image) = player_res.unwrap();
        let player_size = assets.get(player_image).unwrap().size();
        let scaled_player_size = player_size.as_vec2() * player.scale.truncate();
        for (transform, image) in env.iter_mut() {
            let image_size = assets.get(image).unwrap().size();
            let scaled_image_size = image_size.as_vec2() * transform.scale.truncate();
            let collision = collide(
                player.translation,
                scaled_player_size,
                transform.translation,
                scaled_image_size,
            );
            if collision.is_some() {
                match collision.unwrap() {
                    Collision::Bottom => {
                        player.translation.y -= (player.translation.y + (scaled_player_size.y * 0.5)) - (transform.translation.y - (scaled_image_size.y * 0.5));
                    }
                    Collision::Left => {
                        player.translation.x -= (player.translation.x + (scaled_player_size.x * 0.5)) - (transform.translation.x - (scaled_image_size.x * 0.5));
                    }
                    Collision::Right => {
                        player.translation.x -= (player.translation.x - (scaled_player_size.x * 0.5)) - (transform.translation.x + (scaled_image_size.x * 0.5));
                    }
                    Collision::Top => {
                        player.translation.y -= (player.translation.y - (scaled_player_size.y * 0.5)) - (transform.translation.y + (scaled_image_size.y * 0.5));
                    }
                    Collision::Inside => {}
                }
            }
        }
    }
}
