mod systems;
use bevy::prelude::*;
use systems::*;

use crate::components::{DebugState, Labels, SelectedUiEntity, SelectedUiMode};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedUiEntity(None, None));
        app.insert_resource(SelectedUiMode("XY".to_string()));
        app.add_systems(OnEnter(DebugState::Debug), setup_debug)
            .add_systems(OnEnter(DebugState::Editor), setup_editor)
            .add_systems(OnExit(DebugState::Debug), clear_ui)
            .add_systems(OnExit(DebugState::Editor), clear_ui)
            .add_systems(Update, switch_state)
            .add_systems(
                FixedUpdate,
                debug_text
                    .after(Labels::PhysicsMove)
                    .run_if(state_exists_and_equals::<DebugState>(DebugState::Debug)),
            )
            .add_systems(
                Update,
                (mouse_scroll, button_system, update_list, move_items).run_if(state_exists_and_equals::<
                    DebugState,
                >(
                    DebugState::Editor
                )),
            );
    }
}
