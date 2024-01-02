mod systems;

use self::systems::*;
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(FixedUpdate, add_gravity);
        app.add_systems(FixedUpdate, check_collision);
    }
}
