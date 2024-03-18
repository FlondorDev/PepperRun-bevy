
use bevy::{asset::LoadedFolder, prelude::*};
use crate::structs::LevelSchema;

#[derive(Resource)]
pub struct SelectedUiEntity(pub Option<Entity>, pub Option<Entity>);

#[derive(Resource)]
pub struct SelectedUiMode(pub String);

#[derive(Resource)]
pub struct AssetsLoading(pub Vec<Handle<LoadedFolder>>, pub bool);

#[derive(Resource)]
pub struct CurrentLevel(pub Option<String>, pub Option<LevelSchema>);
