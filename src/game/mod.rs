mod systems;

use crate::components::{ApplicationState, Labels};

use self::systems::*;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ApplicationState>();
        app.add_systems(Startup, camera::setup)
            .add_systems(
                Update,
                (player::move_player, pepper::move_pepper)
                    .in_set(Labels::Input)
                    .before(Labels::Physics),
            )
            .add_systems(
                FixedUpdate,
                (
                    physics::add_gravity,
                    player::player_wall_collision,
                    physics::move_entity,
                    pepper::player_pepper_collision,
                )
                    .in_set(Labels::Physics)
                    .after(Labels::Input),
            )
            .add_systems(
                Update,
                camera::update_camera
                    .in_set(Labels::Camera)
                    .after(Labels::Physics)
            );
    }
}
