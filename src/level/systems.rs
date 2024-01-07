use std::fs;

use bevy::{prelude::*, render::mesh::VertexAttributeValues, sprite::MaterialMesh2dBundle};

use crate::components::{Collider, CurrentLevel, Gravity, Level, Player, Wall};

use super::utils::position_to_world;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    images: Res<Assets<Image>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut current_level: ResMut<CurrentLevel>,
) {
    let levels = fs::read("assets/levels/level.json").unwrap();

    let level = serde_json::from_str::<Level>(String::from_utf8(levels).unwrap().as_str()).unwrap();

    let position = position_to_world(level.player.as_vec2(), Vec2::ONE);

    commands.spawn((
        SpriteBundle {
            texture: asset_server.get_handle("Player.png").unwrap(),
            transform: Transform::from_xyz(position.x, position.y, 0.),
            ..default()
        },
        Gravity,
        Player {
            speed_mult: 1.,
            jumps: 2,
        },
        Collider {
            is_grounded: false,
            velocity: Vec2::ZERO,
        },
    ));

    level.schema.iter().for_each(move |schema| {
        let texture: Handle<Image> = asset_server.get_handle(schema.texture.clone()).unwrap();
        let texture_size: &Image = images.get(texture.id()).unwrap();
        let size = schema.size.as_vec2();
        let position = position_to_world(schema.position.as_vec2(), size);
        let mut mesh: Mesh = shape::Quad {
            size: size * texture_size.size().as_vec2(),
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
                transform: Transform::from_xyz(position.x, position.y, 0.),
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

    current_level.0 = Some("level".to_string());
    current_level.1 = Some(level);
}
