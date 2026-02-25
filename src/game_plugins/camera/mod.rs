use bevy::prelude::*;

pub mod systems;
use systems::*;


pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostUpdate, rotate_cam_from_look_input)
            ;
    }
}


