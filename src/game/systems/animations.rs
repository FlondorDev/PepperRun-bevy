use crate::structs::components::{AnimationIndices, AnimationTimer, Oscillante};
use bevy::prelude::{Query, Res, TextureAtlas, Time, Transform};

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

pub fn move_oscillante(
    mut pepper_query: Query<(&mut Oscillante, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut _oscillante, mut transform) in &mut pepper_query {
        transform.translation.y += (time.elapsed_seconds() * 2.2).sin() * 0.2;
    }
}
