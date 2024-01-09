
use bevy::{prelude::*, render::mesh::VertexAttributeValues};
use serde::{Deserialize, Serialize};

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
pub enum ApplicationState {
    #[default]
    LoadingAssets,
    AssetsLoaded,
    Game,
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
pub enum DebugState {
    #[default]
    None,
    Debug,
    Editor,
}

#[derive(Component)]
pub struct DebugUI(pub DebugState);

#[derive(Resource)]
pub struct SelectedUiEntity(pub Option<Entity>, pub Option<Entity>);

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

#[derive(Component)]
pub struct Collider {
    pub is_grounded: bool,
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct Level;

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

#[derive(Resource)]
pub struct CurrentLevel(pub Option<String>, pub Option<LevelSchema>);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LevelSchema {
    pub schema: Vec<ObjectSchema>,
    pub player: Vec2Ser,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ObjectSchema {
    pub position: Vec2Ser,
    pub texture: String,
    pub size: Vec2Ser,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Vec2Ser {
    pub x: f32,
    pub y: f32,
}

impl Vec2Ser {
    pub fn as_vec2(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}

pub trait PositionToVec2 {
    fn size(&self) -> Vec2;
    fn flip_uv(&mut self, flip_x: bool);
}

impl PositionToVec2 for Mesh {
    fn size(&self) -> Vec2 {
        match self.attribute(Mesh::ATTRIBUTE_POSITION).unwrap() {
            VertexAttributeValues::Float32x3(values) => {
                Vec2::new(values[0][0].abs() * 2., values[0][1].abs() * 2.)
            }
            _ => Vec2::ZERO,
        }
    }

    fn flip_uv(&mut self, flip_x: bool) {
        match self.attribute_mut(Mesh::ATTRIBUTE_UV_0).unwrap() {
            VertexAttributeValues::Float32x2(values) => {
                for uv in values.iter_mut() {
                    if flip_x && uv[0].is_sign_positive() || !flip_x && uv[0].is_sign_negative() {
                        uv[0] *= -1.;
                    }
                }
            }
            _ => {}
        }
    }
}
