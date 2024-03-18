mod systems;


use self::systems::*;
use bevy::prelude::*;
use crate::structs::states::{ApplicationState, Labels};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ApplicationState>();
        app.add_systems(Startup, camera::setup)
            .add_systems(
                Update,
                (
                    player::move_player,
                    pepper::move_pepper,
                    animations::animate_sprite,
                )
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
                    camera::update_camera,
                )
                    .chain()
                    .in_set(Labels::Physics)
                    .after(Labels::Input),
            );
    }
}
