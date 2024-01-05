mod systems;

use crate::components::Labels;

use self::systems::*;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(
                Update,
                (move_player)
                    .in_set(Labels::Input)
                    .before(Labels::PhysicsGravity),
            )
            .add_systems(
                FixedUpdate,
                player_wall_collision
                    .in_set(Labels::PhysicsCollision)
                    .after(Labels::PhysicsGravity)
                    .before(Labels::PhysicsMove),
            );
    }
}
