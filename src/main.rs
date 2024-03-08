mod camera;
mod components;
mod debug;
mod level;
mod loading;
mod physics;
mod player;
mod pepper;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use components::{ApplicationState, DebugState};

fn main() {
    App::new()
        .init_state::<ApplicationState>()
        .init_state::<DebugState>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(loading::LoadingPlugin)
        .add_plugins(level::LevelPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(pepper::PepperPlugin)
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(debug::DebugPlugin)
        .run();
}
