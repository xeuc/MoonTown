use bevy::prelude::*;

use bevy_rapier3d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use systems::*;

pub mod components;
pub mod systems;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
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
            .add_systems(Startup, setup_player_camera_integrated)
            .add_systems(Update, translate_player)
            ;
    }
}


