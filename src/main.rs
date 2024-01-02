mod components;
mod player;
mod camera;
mod debug;
mod physics;


use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            ImagePlugin::default_nearest(),
        ))
        .add_plugins(player::PlayerPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(debug::DebugPlugin)
        .run();
}

