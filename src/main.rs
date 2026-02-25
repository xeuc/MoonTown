use bevy::prelude::*;

mod game_plugins;
use crate::game_plugins::GamePlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugins)
        .run();
}


// # Features priorities
// Numeric movment
//     Conf file
//     Pause and change Pause
//     Save file
// Split Screen
// Multi local
//     Gamepad controller
//     Animation
// Trigger's Button
//     Trigger's Area
// Multi-scene load
//     Sparkling0's UI
// TP
// Scene Travel
//     Grass
//     Shaders
//     During a tp in another zone, the camera unzooms,
//      with Sparkling's loading screen,
//      on a scene that represents the world map,
//      and starts from where the player is located
//      and goes to the destination, the player tp, and that's it.

// subsurface scattering
// parallax mapping
// Billboarding

// IES Light
// Motion blurr
// Camera shacking