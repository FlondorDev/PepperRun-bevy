use bevy::prelude::*;

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
pub enum ApplicationState {
    #[default]
    LoadingAssets,
    AssetsLoaded,
    Game,
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
pub enum DebugState {
    #[default]
    None,
    Debug,
    Editor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum Labels {
    Input,
    Physics,
}