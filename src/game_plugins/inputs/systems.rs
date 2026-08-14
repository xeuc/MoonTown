use bevy::input::mouse::MouseMotion;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy::prelude::*;
use crate::game_plugins::shared::components::*;


// STAUTS


/// ******************************************************************************
/// ***  MISC  *******************************************************************
/// ******************************************************************************

// make cursor don't move and invisible
pub fn cursor_grab( 
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let Ok(mut primary_window) = q_windows.single_mut() else { return;};
    // primary_window.cursor_options.grab_mode = CursorGrabMode::Confined;
    primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor_options.visible = false;
}









// UPDATES

// keyboard  => movement_input: ResMut<MovementInput> => for translate_player_and_cam()
// mouse pos => look_input:     ResMut<LookInput>     => for rotate_cam_from_look_input()
pub fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut movement_input: ResMut<MovementInput>,
    mut look_input: ResMut<LookInput>,
    mut mouse_events: EventReader<MouseMotion>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if keyboard.pressed(KeyCode::KeyW) {
        movement_input.z -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        movement_input.z += 1.0
    }
    if keyboard.pressed(KeyCode::KeyA) {
        movement_input.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        movement_input.x += 1.0
    }
    **movement_input = movement_input.normalize_or_zero();
    if keyboard.pressed(KeyCode::Space) {
        **movement_input *= 2.0;
    }
    if keyboard.pressed(KeyCode::ShiftLeft) {
        movement_input.y = 1.0;
    }

    if keyboard.pressed(KeyCode::Escape) {
        app_exit_events.write(AppExit::Success);
    }

    for event in mouse_events.read() {
        look_input.x -= event.delta.x * MOUSE_SENSITIVITY;
        look_input.y -= event.delta.y * MOUSE_SENSITIVITY;
        look_input.y = look_input.y.clamp(-89.9, 89.9); // Limit pitch
    }
}

