use crate::structs::bundles::BaseBundleColl;
use crate::structs::components::{Ice, Milk, Oscillante, Pepper, Spike, Wall};
use crate::structs::{ObjectSchema, PositionToVec2};
use bevy::prelude::Rectangle;
use bevy::{
    asset::{AssetServer, Assets, Handle},
    ecs::system::{Commands, Res, ResMut},
    math::{vec2, Vec2},
    render::{mesh::Mesh, texture::Image},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    transform::components::Transform,
};

#[inline]
pub fn position_to_world(position: Vec2, size: Vec2) -> Vec2 {
    (position * vec2(64., 64.)) + (size * vec2(64., 64.) * 0.5)
}

pub fn generate_mesh2d(
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    images: &Res<Assets<Image>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    schema: &ObjectSchema,
) -> MaterialMesh2dBundle<ColorMaterial> {
    let texture: Handle<Image> = asset_server
        .get_handle(format!("texture/{}", schema.texture.clone()))
        .unwrap();
    let texture_size: &Image = images.get(texture.id()).unwrap();
    let size = schema.size.as_vec2();
    let position = position_to_world(schema.position.as_vec2(), size);
    let mut mesh: Mesh = Rectangle {
        half_size: (size * texture_size.size().as_vec2()) * 0.5,
    }
    .into();
    mesh.set_uv_size(size * 64.);

    let mesh_handle = meshes.add(mesh);

    let material: Handle<ColorMaterial> = materials.add(texture);

    MaterialMesh2dBundle {
        mesh: mesh_handle.into(),
        transform: Transform::from_xyz(position.x, position.y, 0.),
        material,
        ..Default::default()
    }
}

pub fn spawn_object(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    images: &Res<Assets<Image>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    schema: &ObjectSchema,
) {
    let mesh = generate_mesh2d(asset_server, meshes, images, materials, schema);
    let name = schema
        .texture
        .clone()
        .split_once(".")
        .unwrap()
        .0
        .to_string();

    if schema.texture.contains("Pepper") {
        commands.spawn((BaseBundleColl::new(name, mesh), Oscillante, Pepper));
    } else if schema.texture.contains("Spunzone") {
        commands.spawn((BaseBundleColl::new(name, mesh), Spike));
    } else if schema.texture.contains("Latte") {
        commands.spawn((BaseBundleColl::new(name, mesh), Oscillante, Milk));
    } else if schema.texture.contains("Giacchiolo") {
        commands.spawn((BaseBundleColl::new(name, mesh), Oscillante, Ice));
    } else {
        commands.spawn((BaseBundleColl::new(name, mesh), Wall));
    }
}
