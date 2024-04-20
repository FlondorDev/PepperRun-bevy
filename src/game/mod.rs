mod systems;

use self::systems::*;
use crate::structs::resources::{AssetsLoading, Score, UiBarScore};
use crate::structs::states::{ApplicationState, Labels, PlayerState};
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ApplicationState>()
            .init_state::<PlayerState>();

        app.insert_resource(UiBarScore::default())
            .insert_resource(Score::default());

        app.add_systems(Startup, camera::setup)
            .add_systems(OnEnter(PlayerState::None), ui::setup)
            .add_systems(
                Update,
                (
                    player::move_player,
                    animations::move_oscillante,
                    animations::animate_sprite,
                    player::player_death.run_if(in_state(PlayerState::Dead)),
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
                )
                    .chain()
                    .in_set(Labels::Physics)
                    .after(Labels::Input),
            )
            .add_systems(
                FixedUpdate,
                (
                    camera::update_camera,
                    pepper::player_pepper_collision,
                    spike::player_spike_collision,
                    milk::player_milk_collision.run_if(in_state(PlayerState::Pepper)),
                    ice::player_ice_collision.run_if(in_state(PlayerState::Pepper)),
                    ui::update_ui.run_if(in_state(PlayerState::Pepper)),
                )
                    .in_set(Labels::Other)
                    .after(Labels::Physics),
            );
    }
}
