mod camera;
mod components;
mod debug;
mod physics;
mod player;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(player::PlayerPlugin)
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(debug::DebugPlugin)
        .run();
}
