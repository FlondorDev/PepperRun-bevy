mod systems;
use bevy::prelude::*;
use systems::*;

use crate::components::Labels;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(FixedUpdate, debug_text.after(Labels::PhysicsMove));
    }
}
