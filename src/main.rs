mod camera;
mod components;
mod debug;
mod level;
mod loading;
mod physics;
mod player;

use bevy::prelude::*;
use components::ApplicationState;

fn main() {
    App::new()
        .add_state::<ApplicationState>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(loading::LoadingPlugin)
        .add_plugins(level::LevelPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(debug::DebugPlugin)
        .run();
}
