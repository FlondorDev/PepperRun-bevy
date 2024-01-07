use std::fs;

use bevy::{prelude::*, render::mesh::VertexAttributeValues, sprite::MaterialMesh2dBundle};

use crate::components::{Collider, Level, Wall, PositionToVec2};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let levels = fs::read("assets/levels/level.json").unwrap();

    let level = serde_json::from_str::<Level>(String::from_utf8(levels).unwrap().as_str()).unwrap();

    level.schema.iter().for_each(|schema| {
        let size = schema.size.as_vec2();
        let mut mesh: Mesh = shape::Quad {
            size: size * 64.,
            ..default()
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

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                transform: Transform::from_xyz(schema.position.x, schema.position.y, 0.),
                material: materials.add(
                    asset_server
                        .get_handle(schema.texture.clone())
                        .unwrap()
                        .into(),
                ),
                ..default()
            },
            Collider {
                is_grounded: false,
                velocity: Vec2::ZERO,
            },
            Wall,
        ));
    });
}
