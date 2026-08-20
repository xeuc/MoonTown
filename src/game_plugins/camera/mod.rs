use bevy::prelude::*;

pub mod systems;
use systems::*;

use crate::game_plugins::player::switch_how_camera_move::CameraControlMode;


pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostUpdate, rotate_cam_from_look_input.run_if(resource_equals(CameraControlMode::WithCursor)))
            ;
    }
}


