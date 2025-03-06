use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::plugin::TimestepMode;

pub mod plugins;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FrameTimeDiagnosticsPlugin,
            WorldInspectorPlugin::new(),
        ))
        .add_plugins((
            plugins::lost_player::LostPlayerPlugin,
            plugins::screenshot::ScreenshotPlugin,
            plugins::cursor_as_movement::RotateHeadPlugin,
            plugins::pause::PausePlugin,
            plugins::keyboard::KeyboardPlugin,
            plugins::skybox::SkyboxPlugin,
            plugins::light::LightPlugin,
            plugins::fps::FpsPlugin,
            plugins::egui::UiPlugin,
            plugins::app_state::AppStatePlugin,
            plugins::setup_map::SetupMapPlugin,
            plugins::player::controls_player_ball::ControlsPlayerBallPlugin,
            plugins::player::spawn_player_ball::SpawnPlayerBallPlugin,
            plugins::debug::DebugPlugin,
            plugins::pokeball::SpawnPokeBallPlugin,
        ))
        .insert_resource(TimestepMode::Fixed {
            dt: 1.0 / 240.0,
            substeps: 64,
        })// Clip using "f" key
        .run();
}



// TODO
// BUG
// * les plugins devrait pas s'appeler les uns les autres, mais plutot utiliser des querries (genge skybox est appelé par setup_maps)
// fix "ERROR present_frames: log: No work has been submitted for this frame" when taking screenshot (to be continued)

// MAP
// import camera as gltf
// import light as gltf
// save gltf modified by game map
// Blennder same dynamic scene https://www.youtube.com/watch?v=PRDUH0JUS_A
// [OPTI] MAKE check_mesh_ready_no_rapier CALLED ONCE (or not avter the thing being done)
// [OPTI] ne pas voir ce qui est derriere sois 

// FEATURE
// Config file
// split screen
// threads?
// portail like realist nether portail
// dimention 

// COLLISION
// Au lieu d'utiliser des collision group, juste, désactive ceux qui sont pas dans le chunk
// stp fix le perso savonette
// rapier https://rapier.rs/docs/user_guides/bevy_plugin/getting_started_bevy/

    // DONE
    // import gltf background
    // finish the tuto
    // change all to plugin 
    // créer un plugin pour setup comme ça le main fait 100 lignes :)
    // y<1 => TP player
    // enlever le publique (pub) des plugins
    // UI log position + DEBUG
    // remet la lumiere stp
    // remettre ui mais en tant que plugin
    // fix screenshot folder not exist error




// https://github.com/bevyengine/bevy/discussions/5522
// egui is an immediate mode UI library.
// This means that the state of the UI is built from scratch every frame,
// but it also means the state is entirely transparent, without the need for propagating state through a retained tree.
// In bevy, using bevy_egui, this state is accessed through the EguiContext resource - a single struct that contains the entire UI state.
// When you have mutable access to the EguiContext, you can add widgets, updating the state.




// Do INDEPENDANT plugins or look for plugins dependancies

// Perfs:
//           | ⧖ debug plugin | ϟ no debug plugin |
// ⧖ no opti |         10 fps |           100 fps |
// ϟ -o3     |        140 fps |           140 fps |


// change map 
// Resolve light problem
// generate tree procedurally ? (No)