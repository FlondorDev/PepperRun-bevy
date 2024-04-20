use crate::structs::LevelSchema;
use bevy::{asset::LoadedFolder, prelude::*};

#[derive(Resource)]
pub struct SelectedUiEntity(pub Option<Entity>, pub Option<Entity>);

#[derive(Resource)]
pub struct SelectedUiMode(pub String);

#[derive(Resource)]
pub struct AssetsLoading(pub Vec<Handle<LoadedFolder>>, pub bool);

#[derive(Resource)]
pub struct CurrentLevel(pub Option<String>, pub Option<LevelSchema>);

#[derive(Resource, Deref, DerefMut, Default)]
pub struct Score(pub f32);

#[derive(Resource, Deref, DerefMut)]
pub struct UiBarScore(pub f32);
impl Default for UiBarScore {
    fn default() -> Self {
        UiBarScore(100.)
    }
}
