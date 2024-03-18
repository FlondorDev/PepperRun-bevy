use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;
use bevy::render::primitives::Aabb;

use crate::game::systems::player::PEPPER_SPEED_MULTIPLIER;
use crate::structs::components::{AnimationIndices, AnimationTimer, Level, Oscillante, Pepper, Player};

pub fn move_pepper(mut pepper_query: Query<(&mut Oscillante, &mut Transform)>, time: Res<Time>) {
    for (mut _oscillante, mut transform) in &mut pepper_query {
        transform.translation.y += (time.elapsed_seconds() * 2.2).sin() * 0.2;
    }
}

pub fn player_pepper_collision(
    mut player_query: Query<
        (&Transform, &Aabb, &mut Player, Entity),
        (With<Player>, Without<Pepper>),
    >,
    mut pepper_query: Query<(Entity, &Pepper, &Aabb, &mut Transform)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let player_res = player_query.get_single_mut();
    if player_res.is_ok() {
        let (player_transform, player_aabb, mut player, player_entity) = player_res.unwrap();
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
                let texture: Handle<Image> = asset_server
                    .get_handle(format!("texture/{}", "FuocoB.png"))
                    .unwrap();
                let layout = TextureAtlasLayout::from_grid(Vec2::new(66.0, 66.0), 5, 1, None, None);
                let texture_atlas_layout = texture_atlas_layouts.add(layout);

                let animation_indices = AnimationIndices { first: 0, last: 4 };

             

                commands
                    .get_entity(player_entity)
                    .unwrap()
                    .with_children(|commands| {
                        let mut transform = Transform::from_xyz(-45., -10., 0.); //player_transform.clone();
                        transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
                        commands.spawn((
                            SpriteSheetBundle {
                                texture,
                                atlas: TextureAtlas {
                                    layout: texture_atlas_layout,
                                    index: animation_indices.first,
                                },
                                transform,
                                ..default()
                            },
                            Level,
                            //Target(player_entity),
                            //Offset(Vec2::new(-45.0, -10.)),
                            animation_indices,
                            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                        ));
                    });

                commands.entity(pepper_entity).despawn();
                player.speed_mult *= PEPPER_SPEED_MULTIPLIER;
            }
        }
    }
}
