mod systems;

use crate::components::{Labels, ApplicationState};

use self::systems::*;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_camera.in_set(Labels::Camera).after(Labels::PhysicsMove).after(Labels::Input));
    }
}
