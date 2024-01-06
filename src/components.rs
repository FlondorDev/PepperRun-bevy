use bevy::{prelude::*, sprite::collide_aabb::Collision};

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Gravity;

#[derive(Component)]
pub struct Player {
    pub speed_mult: f32,
    pub jumps: i8,
}

#[derive(Component)]
pub struct Collider {
    pub is_grounded: bool,
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct Wall;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum Labels {
    Input,
    PhysicsGravity,
    PhysicsCollision,
    PhysicsMove,
    Camera,
}

