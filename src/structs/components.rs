use bevy::prelude::*;
use crate::structs::states::DebugState;

#[derive(Component)]
pub struct DebugUI(pub DebugState);

#[derive(Component)]
pub struct UiEntityRef(pub Entity);

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Gravity;

#[derive(Component)]
pub struct Player {
    pub speed_mult: f32,
    pub jumps: i8,
}

#[derive(Component, Default)]
pub struct Collider {
    pub is_grounded: bool,
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);


#[derive(Component)]
pub struct CameraHighLow(pub Vec2, pub Vec2);


#[derive(Component)]
pub struct Pepper;

#[derive(Component)]
pub struct Oscillante;

#[derive(Component)]
pub struct Level;

#[derive(Component)]
pub struct Wall;