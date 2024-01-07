use bevy::math::{vec2, Vec2};

#[inline]
pub fn position_to_world(position: Vec2, size: Vec2) -> Vec2 {
    position * vec2(64., 64.) + size * vec2(64., 64.) * 0.5
}