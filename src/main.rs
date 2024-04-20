mod asset_loader;
mod debug;
mod game;
mod level;
mod structs;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        //.add_plugins(WorldInspectorPlugin::new())
        .add_plugins(asset_loader::AssetLoaderPlugin)
        .add_plugins(level::LevelPlugin)
        .add_plugins(game::GamePlugin)
        .add_plugins(debug::DebugPlugin)
        .run();
}
