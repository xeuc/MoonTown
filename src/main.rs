use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod plugins;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FrameTimeDiagnosticsPlugin,
            // WorldInspectorPlugin::new(),
        ))
        .add_plugins((
            plugins::lost_player::LostPlayerPlugin,
            plugins::screenshot::ScreenshotPlugin,
            plugins::cursor_as_movement::RotateHeadPlugin,
            plugins::pause::PausePlugin,
            plugins::keyboard::KeyboardPlugin,
            plugins::skybox::SkyboxPlugin,
            plugins::light::LightPlugin,
            // plugins::fps::FpsPlugin,
            // plugins::egui::UiPlugin,
            plugins::app_state::AppStatePlugin,
            plugins::setup_map::SetupMapPlugin,
            plugins::player::spawn_player_ball::SpawnPlayerBallPlugin,
            // plugins::debug::DebugPlugin,
            plugins::pokeball::SpawnPokeBallPlugin,
        ))
        .add_plugins((
            plugins::animation::AnimationPlugin,
            plugins::player::controls_player_ball::ControlsPlayerBallPlugin,
            plugins::iyes_perf_ui::IvesPerfUIPlugin,
        ))

        // .insert_resource(TimestepMode::Fixed {
        //     dt: 1.0 / 240.0,
        //     substeps: 64,
        // })// Clip using "f" key
        .run();
}

// TODO
// BUG
// * Plugins should not call each other, but rather use queries (e.g., skybox is called by setup_maps)
// Fix "ERROR present_frames: log: No work has been submitted for this frame" when taking a screenshot (to be continued)

// MAP
// Import camera as glTF
// Import light as glTF
// Save glTF modified by game map
// Blender same dynamic scene https://www.youtube.com/watch?v=PRDUH0JUS_A
// [OPTI] Do not see what is behind you

// FEATURE
// Config file
// Split screen
// Threads?
// Portal-like realistic Nether portal
// Dimension

// COLLISION
// Instead of using collision groups, just disable those that are not in the chunk
// Please fix the slippery character
// Rapier https://rapier.rs/docs/user_guides/bevy_plugin/getting_started_bevy/

    // DONE
    // Import glTF background (glTF file itself does not directly support a background)
    // Finish the tutorial (???)
    // Change everything to plugins
    // Create a plugin for setup so that main is only 100 lines :)
    // y < 1 => TP player
    // Remove public (pub) from plugins
    // UI log position + DEBUG
    // Please put the light back
    // Put UI back but as a plugin
    // Fix screenshot folder not existing error
    // [OPTI] MAKE check_mesh_ready_no_rapier CALLED ONCE (or not after the thing is done) (gamestate or component)
    // Resolve light problem
    // Generate trees procedurally? (No)

// https://github.com/bevyengine/bevy/discussions/5522
// egui is an immediate mode UI library.
// This means that the state of the UI is built from scratch every frame,
// but it also means the state is entirely transparent, without the need for propagating state through a retained tree.
// In Bevy, using bevy_egui, this state is accessed through the EguiContext resource - a single struct that contains the entire UI state.
// When you have mutable access to the EguiContext, you can add widgets, updating the state.

// Create INDEPENDENT plugins or look for plugin dependencies

// Performance:
//           | ⧖ debug plugin | ϟ no debug plugin |
// ⧖ no opti |         10 fps |           100 fps |
// ϟ -o3     |        140 fps |           140 fps |

// Change map

// we should not draw polygons whose normals match the camera's view direction.


// Create a Blockbench plugin to allow "extras" like Blender do with "custom properties" that can be reached from bevy using GltfExtras
// camera should turn arond player without player turning as well 

