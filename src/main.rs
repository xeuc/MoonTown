use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, input::InputSystem, prelude::*
};
use bevy_rapier3d::{prelude::*};



mod spawners;
mod utils;
mod update;
use utils::MovementInput;
use utils::LookInput;
use update::{handle_input, translate_player, rotate_cam_from_look_input, rotate_button_behavior, resize_little_rectangle};
// use bevy_egui::EguiPlugin;

// use bevy_inspector_egui::quick::WorldInspectorPlugin;

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

            // EguiPlugin::default(),
            // WorldInspectorPlugin::new(),
            
            // The other diagnostics plugins can still be used without this if you want to use them in an ingame overlay for example.
            // LogDiagnosticsPlugin::default(),
            // // Adds frame time, FPS and frame count diagnostics.
            // FrameTimeDiagnosticsPlugin::default(),
            // // Adds an entity count diagnostic.
            // bevy::diagnostic::EntityCountDiagnosticsPlugin,
            // // Adds cpu and memory usage diagnostics for systems and the entire game process.
            // bevy::diagnostic::SystemInformationDiagnosticsPlugin,
            // Forwards various diagnostics from the render app to the main app.
            // These are pretty verbose but can be useful to pinpoint performance issues.
            // bevy_render::diagnostic::RenderDiagnosticsPlugin,

            
        ))
        .add_systems(Startup, (
            spawners::setup_player_camera_integrated,
            spawners::setup_maps_elements,
            spawners::setup_ui,
            spawners::cursor_grab,
        ))

        .add_systems(PreUpdate, handle_input.after(InputSystem))
        .add_systems(FixedUpdate, rotate_button_behavior)
        .add_systems(Update, (
            translate_player,
            rotate_cam_from_look_input,
            resize_little_rectangle,
        ))

        .run();
}

// TODO: Fix scheduler:
//       from https://bevy-cheatbook.github.io/fundamentals/fixed-timestep.html
//       The following things should probably be done in FixedUpdate:
//         Physics and collision detection
//         Networking / netcode
//         AI for enemies and NPCs (pathfinding, decisions, etc.)
//         Spawning/despawning gameplay-related entities
//         Other simulation and decision-making
//       The following things should probably be done in Update:        
//         Camera movement and controls
//         Animations
//         UI
//         Visual effects
//         Anything that is part of your game's graphics/visuals or interactivity
//         App state transitions


