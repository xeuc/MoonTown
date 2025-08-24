use bevy::prelude::*;





pub mod systems;
use crate::game_plugins::ui::systems::*;
pub mod components;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                setup_ui,
            ))
            .add_systems(FixedUpdate, rotate_button_behavior)
            .add_systems(Update, (
                resize_little_rectangle,
            ))
            ;


    }
}


