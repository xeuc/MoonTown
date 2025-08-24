use bevy::prelude::*;
use bevy::{input::InputSystem};


pub mod systems;
use systems::*;
use crate::game_plugins::shared::components::*;

pub struct InputsPlugin;
impl Plugin for InputsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MovementInput>()
            .init_resource::<LookInput>()
            .add_systems(Startup, cursor_grab)
            .add_systems(PreUpdate, handle_input.after(InputSystem))
            ;
    }
}


