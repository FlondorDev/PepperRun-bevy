mod systems;
pub mod utils;

use self::systems::*;
use crate::structs::resources::CurrentLevel;
use crate::structs::states::ApplicationState;
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentLevel(None, None));
        app.add_systems(OnEnter(ApplicationState::AssetsLoaded), setup);
        app.add_systems(Update, despawn_all.run_if(in_state(ApplicationState::Game)));
    }
}
