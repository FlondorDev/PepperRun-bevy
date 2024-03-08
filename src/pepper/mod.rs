mod systems;

use crate::components::Labels;

use self::systems::*;
use bevy::prelude::*;

pub struct PepperPlugin;

impl Plugin for PepperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            move_pepper
        )
        .add_systems(
            FixedUpdate,
            player_pepper_collision
        );
    }
}
