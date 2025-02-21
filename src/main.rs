use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;


mod plugins {
    pub mod lost_player;
    pub mod fps; 
    pub mod screenshot;
    pub mod char_controller;
    pub mod pause;
    pub mod cursor_as_movement;
    pub mod keyboard;
    pub mod setup_map;
    pub mod skybox;
    pub mod light;
    pub mod additional_crate_ui;
    pub mod app_state;
    pub mod spawn_terrain;
    // pub mod load_gltf_one_poly_at_a_time;
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FrameTimeDiagnosticsPlugin,
            plugins::char_controller::CharacterControllerPlugin,
            plugins::lost_player::LostPlayerPlugin,
            plugins::screenshot::ScreenshotPlugin,
            plugins::cursor_as_movement::RotateHeadPlugin,
            plugins::pause::PausePlugin,
            plugins::keyboard::KeyboardPlugin,
            plugins::setup_map::SetupMapPlugin,
            plugins::skybox::SkyboxPlugin,
            plugins::light::LightPlugin,
            plugins::fps::FpsPlugin,
            plugins::additional_crate_ui::UiPlugin,
            plugins::app_state::AppStatePlugin,
            // plugins::spawn_terrain::SpawnTerrainPlugin,
            // plugins::load_gltf_one_poly_at_a_time::LoadGLTF2Plugin,
        ))
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