use bevy::math::Quat;
use bevy::prelude::{Camera, Color, Gizmos, Query, Res, Time, Transform, Window, With, Without};
use bevy::render::primitives::Aabb;
use bevy::window::PrimaryWindow;
use crate::structs::components::Bg;

fn update_bg(
    mut q_bg: Query<(&mut Transform, &mut Bg, &Aabb), Without<Camera>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<&Transform, (With<Camera>, Without<Bg>)>,
) {
    for (mut transform, mut boid, aabb) in &mut q_bg {
        let camera_transform = q_camera.single();
        let screen = q_windows.single();
        
        
         
    }
}