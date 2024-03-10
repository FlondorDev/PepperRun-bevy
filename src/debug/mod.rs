mod systems;
use bevy::prelude::*;
use systems::*;

use crate::components::{DebugState, Labels, SelectedUiEntity, SelectedUiMode};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<DebugState>();

        app.insert_resource(SelectedUiEntity(None, None));
        app.insert_resource(SelectedUiMode("XY".to_string()));

        app.add_systems(Update, switch_state)
            // DEBUG MENU
            .add_systems(OnEnter(DebugState::Debug), setup_debug)
            .add_systems(
                Update,
                debug_text.run_if(in_state(DebugState::Debug)).after(Labels::Physics),
            )
            .add_systems(OnExit(DebugState::Debug), clear_ui)
            // EDITOR MENU
            .add_systems(OnEnter(DebugState::Editor), setup_editor)
            .add_systems(
                Update,
                (
                    mouse_scroll,
                    reset_button,
                    button_system,
                    update_list,
                    move_items,
                )
                    .run_if(in_state(DebugState::Editor)),
            )
            .add_systems(OnExit(DebugState::Editor), clear_ui);
    }
}
