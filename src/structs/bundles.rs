use crate::structs::components::{Collider, Level, Name};
use bevy::math::Vec2;
use bevy::prelude::Bundle;
use bevy::sprite::{ColorMaterial, MaterialMesh2dBundle};

#[derive(Bundle)]
pub struct BaseBundleColl {
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
    pub collider: Collider,
    pub level: Level,
    pub name: Name,
}
impl BaseBundleColl {
    pub fn new(name: String, mesh: MaterialMesh2dBundle<ColorMaterial>) -> BaseBundleColl {
        BaseBundleColl {
            collider: Collider::default(),
            level: Level,
            name: Name(name),
            mesh,
        }
    }
}

#[derive(Bundle)]
pub struct BaseBundle {
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
    pub level: Level,
    pub name: Name,
}
impl BaseBundle {
    pub fn new(name: String, mesh: MaterialMesh2dBundle<ColorMaterial>) -> BaseBundle {
        BaseBundle {
            level: Level,
            name: Name(name),
            mesh,
        }
    }
}
