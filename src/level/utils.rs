use bevy::{
    asset::{AssetServer, Assets, Handle},
    ecs::system::{Commands, Res, ResMut},
    math::{vec2, Vec2},
    render::{
        mesh::{shape, Mesh, VertexAttributeValues},
        texture::Image,
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    transform::components::Transform,
};

use crate::components::{Collider, Level, Name, ObjectSchema, Wall};

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
    let texture: Handle<Image> = asset_server.get_handle(schema.texture.clone()).unwrap();
    let texture_size: &Image = images.get(texture.id()).unwrap();
    let size = schema.size.as_vec2();
    let position = position_to_world(schema.position.as_vec2(), size);
    let mut mesh: Mesh = shape::Quad {
        size: size * texture_size.size().as_vec2(),
        ..Default::default()
    }
    .into();
    let uvs = mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0).unwrap();

    match uvs {
        VertexAttributeValues::Float32x2(values) => {
            for uv in values.iter_mut() {
                uv[0] *= size.x;
                uv[1] *= size.y;
            }
        }
        _ => (),
    };

    let mesh_handle = meshes.add(mesh);

    let material: Handle<ColorMaterial> = materials.add(
        asset_server
            .get_handle(schema.texture.clone())
            .unwrap()
            .into(),
    );

    MaterialMesh2dBundle {
        mesh: mesh_handle.into(),
        transform: Transform::from_xyz(position.x, position.y, 0.),
        material: material,
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
    commands.spawn((
        mesh,
        Collider {
            is_grounded: false,
            velocity: Vec2::ZERO,
        },
        Wall,
        Level,
        Name(
            schema
                .texture
                .clone()
                .split_once(".")
                .unwrap()
                .0
                .to_string(),
        ),
    ));
}
