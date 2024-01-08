mod systems;
use bevy::prelude::*;
use systems::*;

use crate::components::{ApplicationState, Labels};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ApplicationState::Game), setup)
            // .add_systems(FixedUpdate, debug_text.after(Labels::PhysicsMove));
            .add_systems(Update, mouse_scroll.after(Labels::PhysicsMove))
            .add_systems(Update, button_system)
            .add_systems(Update, update_list);
    }
}
