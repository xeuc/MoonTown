use bevy::prelude::*;


pub mod components;
pub mod systems;
use bevy_rapier3d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use systems::*;

use bevy::{input::InputSystem};

use crate::game_plugins::player::components::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::srgb(
                0xF9 as f32 / 255.0,
                0xF9 as f32 / 255.0,
                0xFF as f32 / 255.0,
            )))
            .init_resource::<MovementInput>()
            .init_resource::<LookInput>()
            .add_plugins((
                RapierPhysicsPlugin::<NoUserData>::default(),
                RapierDebugRenderPlugin::default(),
            ))
            .add_systems(Startup, (
                setup_player_camera_integrated,
                setup_maps_elements,
                setup_ui,
                cursor_grab,
            ))

            .add_systems(PreUpdate, handle_input.after(InputSystem))
            .add_systems(FixedUpdate, rotate_button_behavior)
            .add_systems(PostUpdate, rotate_cam_from_look_input)
            .add_systems(Update, (
                translate_player,
                resize_little_rectangle,
            ));


    }
}


