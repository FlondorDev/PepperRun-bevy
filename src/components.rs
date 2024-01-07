use bevy::{prelude::*, render::mesh::VertexAttributeValues};
use serde::{Deserialize, Serialize};

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

#[derive(Resource)]
pub struct AssetsLoading(pub Vec<Handle<Image>>, pub bool);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Level {
    pub schema: Vec<LevelSchema>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LevelSchema {
    pub position: Vec2Ser,
    pub texture: String,
    pub size: Vec2Ser
}

#[derive(Debug, Clone, Deserialize, Serialize)]

pub struct Vec2Ser {
    pub x: f32,
    pub y: f32
}

impl Vec2Ser {
    pub fn as_vec2(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y
        }
    }
}

pub trait PositionToVec2 {
    fn size(&self) -> Vec2;
}

impl PositionToVec2 for Mesh {
    fn size(&self) -> Vec2 {
        match self.attribute(Mesh::ATTRIBUTE_POSITION).unwrap() {
            VertexAttributeValues::Float32x3(values) => {
                Vec2::new(values[0][0].abs() * 2., values[0][1].abs() * 2.)
            },
            _=>{
                Vec2::ZERO
            }
        }
    }
}