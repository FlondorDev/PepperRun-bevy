use bevy::math::Vec3A;
use bevy::render::primitives::Aabb;
use bevy::{asset::LoadedFolder, prelude::*, render::mesh::VertexAttributeValues};
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

#[derive(Resource)]
pub struct SelectedUiMode(pub String);

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
pub struct Pepper;

#[derive(Component)]
pub struct Oscillante;

#[derive(Component)]
pub struct Level;

#[derive(Component)]
pub struct Wall;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
    Inside,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum Labels {
    Input,
    PhysicsGravity,
    PhysicsCollision,
    PhysicsMove,
    Camera,
}

#[derive(Resource)]
pub struct AssetsLoading(pub Vec<Handle<LoadedFolder>>, pub bool);

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
    fn set_uv_size(&mut self, size: Vec2);
    fn flip_uv(&mut self, flip_x: bool);
}

pub trait SetSize {
    fn set_size(&mut self, size: Vec2);
}

impl SetSize for Aabb {
    fn set_size(&mut self, size: Vec2) {
        self.half_extents = Vec3A::new(size.x * 0.5, size.y * 0.5, 0.);
    }
}

impl SetSize for Mesh {
    fn set_size(&mut self, size: Vec2) {
        if let VertexAttributeValues::Float32x3(values) =
            self.attribute_mut(Mesh::ATTRIBUTE_POSITION).unwrap()
        {
            values[0] = [-(size.x * 0.5), -(size.y * 0.5), 0.];
            values[1] = [-(size.x * 0.5), (size.y * 0.5), 0.];
            values[2] = [(size.x * 0.5), (size.y * 0.5), 0.];
            values[3] = [(size.x * 0.5), -(size.y * 0.5), 0.];
        }

        self.set_uv_size(size);
    }
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

    fn set_uv_size(&mut self, size: Vec2) {
        let uvs = self.attribute_mut(Mesh::ATTRIBUTE_UV_0).unwrap();

        match uvs {
            VertexAttributeValues::Float32x2(uv) => {
                //uv[0][0] = 0;
                uv[0][1] = size.y / 64.;

                // uv[1][0] = 0;
                // uv[1][1] = 0;

                uv[2][0] = size.x / 64.;
                // uv[2][1] = 0;

                uv[3][0] = size.x / 64.;
                uv[3][1] = size.y / 64.;
            }
            _ => (),
        };
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
