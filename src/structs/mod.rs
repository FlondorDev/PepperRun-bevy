use bevy::math::{Vec2, Vec3A};
use bevy::prelude::Mesh;
use bevy::render::mesh::VertexAttributeValues;
use bevy::render::primitives::Aabb;
use serde::{Deserialize, Serialize};

pub mod bundles;
pub mod components;
pub mod resources;
pub mod states;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
    Inside,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LevelSchema {
    pub schema: Vec<ObjectSchema>,
    pub bg: String,
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
            values[0] = [(size.x * 0.5), (size.y * 0.5), 0.];
            values[1] = [-(size.x * 0.5), (size.y * 0.5), 0.];
            values[2] = [-(size.x * 0.5), -(size.y * 0.5), 0.];
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

        if let VertexAttributeValues::Float32x2(uv) = uvs {
            uv[0][0] = size.x / 64.;
            //uv[0][1] = 0;

            // uv[1][0] = 0;
            // uv[1][1] = 0;

            // uv[2][0] = 0;
            uv[2][1] = size.y / 64.;

            uv[3][0] = size.x / 64.;
            uv[3][1] = size.y / 64.;
        };
    }

    fn flip_uv(&mut self, flip_x: bool) {
        if let VertexAttributeValues::Float32x2(values) =
            self.attribute_mut(Mesh::ATTRIBUTE_UV_0).unwrap()
        {
            for uv in values.iter_mut() {
                if flip_x && uv[0].is_sign_positive() || !flip_x && uv[0].is_sign_negative() {
                    uv[0] *= -1.;
                }
            }
        }
    }
}
