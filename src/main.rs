use bevy::{ input::InputSystem, prelude::* };
use bevy_rapier3d::{prelude::*};

mod spawners;
mod utils;
mod update;
use utils::MovementInput;
use utils::LookInput;
use update::{handle_input, translate_player, rotate_cam_from_look_input, rotate_button_behavior, resize_little_rectangle};


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .init_resource::<MovementInput>()
        .init_resource::<LookInput>()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_systems(Startup, (
            spawners::setup_player_camera_integrated,
            spawners::setup_maps_elements,
            spawners::setup_ui,
            spawners::cursor_grab,
        ))

        .add_systems(PreUpdate, handle_input.after(InputSystem))
        .add_systems(FixedUpdate, rotate_button_behavior)
        .add_systems(PostUpdate, rotate_cam_from_look_input)
        .add_systems(Update, (
            translate_player,
            resize_little_rectangle,
        ))
        .run();
}
