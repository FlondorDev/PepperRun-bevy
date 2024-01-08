mod systems;
use bevy::prelude::*;
use systems::*;

use crate::components::{ApplicationState, DebugState, Labels};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(DebugState::Debug), setup_debug)
            .add_systems(OnEnter(DebugState::Editor), setup_editor)
            .add_systems(OnExit(DebugState::Debug),clear_ui)
            .add_systems(OnExit(DebugState::Editor),clear_ui)
            .add_systems(Update, switch_state)
            .add_systems(
                FixedUpdate,
                debug_text
                    .after(Labels::PhysicsMove)
                    .run_if(state_exists_and_equals::<DebugState>(DebugState::Debug)),
            )
            .add_systems(
                Update,
                (mouse_scroll, button_system, update_list).run_if(state_exists_and_equals::<
                    DebugState,
                >(
                    DebugState::Editor
                )),
            );
    }
}
