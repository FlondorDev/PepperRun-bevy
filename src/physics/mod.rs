mod systems;
pub mod utils;

use crate::components::Labels;

use self::systems::*;
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                add_gravity
                    .in_set(Labels::PhysicsGravity)
                    .before(Labels::PhysicsCollision),
                move_entity
                    .in_set(Labels::PhysicsMove)
                    .after(Labels::PhysicsCollision),
            )
                .after(Labels::Input),
        );
    }
}
