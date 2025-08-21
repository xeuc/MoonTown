use bevy::prelude::*;

mod game_plugins;
use crate::game_plugins::GamePlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugins)
        .run();
}
