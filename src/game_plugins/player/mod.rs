use bevy::prelude::*;

use bevy_rapier3d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use systems::*;

use switch_how_camera_move::*;

pub mod components;
pub mod systems;
pub mod switch_how_camera_move;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CameraControlMode>()

            // To move to SKYBOX plugin
            .insert_resource(ClearColor(Color::srgb(
                0xF9 as f32 / 255.0,
                0xF9 as f32 / 255.0,
                0xFF as f32 / 255.0,
            )))

            // To move to phisics plugin
            .add_plugins((
                RapierPhysicsPlugin::<NoUserData>::default(),
                RapierDebugRenderPlugin::default(),
            ))
            .add_systems(Startup, (
                setup_player_camera_integrated,
                setup_control_hint,
            ))
            
            .add_systems(Update,(
                toggle_control_mode,
                translate_player.run_if(resource_equals(CameraControlMode::WithCursor)),
                translate_player_no_cursor.run_if(resource_equals(CameraControlMode::NoCursor)),
            ))
            ;
    }
}


