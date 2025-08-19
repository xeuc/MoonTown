
use bevy::{
    input::mouse::MouseMotion, prelude::*
};
use bevy_rapier3d::{control::KinematicCharacterController, prelude::*};
use bevy::{
    render::camera::Viewport, window::WindowResized,
};

use crate::utils::{Anchor, CameraPosition, Direction, LookInput, MovementInput, Player, RotateCamera, TopLeftCamera, GRAVITY, GROUND_TIMER, JUMP_SPEED, MOUSE_SENSITIVITY, MOVEMENT_SPEED};
use crate::utils::PlayerCamera;




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

















// ACTUALLY TRANSLATE THE PLAYER AND THE CAMERA (6 directions)
// movement_input come from keyboard
pub fn translate_player(
    time: Res<Time>,
    mut movement_input: ResMut<MovementInput>,
    mut anchor: Query<(
        &mut Transform,
        &mut KinematicCharacterController,
        Option<&KinematicCharacterControllerOutput>,
    ), (With<Anchor>, Without<PlayerCamera>),>,
    mut camera: Query<&mut Transform, (With<PlayerCamera>, Without<Anchor>)>,
    mut vertical_movement: Local<f32>,
    mut grounded_timer: Local<f32>,
) {
    let Ok((anchor_transform, mut anchor_controller, anchor_output)) = anchor.single_mut() else { return; };
    let Ok(camera_transform) = camera.single_mut() else { return; };

    let delta_time = time.delta_secs();

    // Get camera's forward and right vectors (ignoring Y for planar movement)
    let mut cam_forward: Vec3 = camera_transform.forward().into();
    cam_forward.y = 0.0;
    cam_forward = cam_forward.normalize_or_zero();

    let mut cam_right: Vec3 = camera_transform.right().into();
    cam_right.y = 0.0;
    cam_right = cam_right.normalize_or_zero();

    // Movement relative to camera
    let input = **movement_input;
    let mut anchor_movement = (-cam_forward * input.z + cam_right * input.x) * MOVEMENT_SPEED;

    let jump_speed = movement_input.y * JUMP_SPEED;
    **movement_input = Vec3::ZERO;

    // Ground check
    if anchor_output.map(|o| o.grounded).unwrap_or(false) {
        *grounded_timer = GROUND_TIMER;
        *vertical_movement = 0.0;
    }
    if *grounded_timer > 0.0 {
        *grounded_timer -= delta_time;
        if jump_speed > 0.0 {
            *vertical_movement = jump_speed;
            *grounded_timer = 0.0;
        }
    }
    anchor_movement.y = *vertical_movement;
    *vertical_movement += GRAVITY * delta_time * anchor_controller.custom_mass.unwrap_or(1.0);

    anchor_controller.translation = Some(anchor_transform.rotation * (anchor_movement * delta_time));


}




















// take the input mouse from look_input, and use it to rotate the camera around the player
pub fn rotate_cam_from_look_input(
    mut camera: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    mut player: Query<&mut Transform, (With<Player>, Without<PlayerCamera>)>,
    mut look_input: ResMut<LookInput>,
) {
    let Ok(mut camera_transform) = camera.single_mut() else { return;};
    // I use player instead of anchor to avoid problem of jitter + problem of player shaking along Y ¯\_(ツ)_/¯
    let Ok(player_transform) = player.single_mut() else { return;};

    let target = player_transform.translation;
    let cam_local_x = camera_transform.right().as_vec3();
    let pitch = Quat::from_axis_angle(cam_local_x, look_input.y.to_radians());
    let yaw = Quat::from_rotation_y(look_input.x.to_radians());
    let rotation = yaw * pitch;
    camera_transform.rotate_around(target, rotation);
    camera_transform.look_at(target, Vec3::Y);

    // Reset look input after applying it
    **look_input = Vec2::ZERO;
}




















// Osef
// Rotate button behavior
pub fn rotate_button_behavior(
    interaction_query: Query<
        (&Interaction, &ComputedNodeTarget, &RotateCamera),
        (Changed<Interaction>, With<Button>),
    >,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    for (interaction, computed_target, RotateCamera(direction)) in &interaction_query {
        if let Interaction::Pressed = *interaction {
            // Since TargetCamera propagates to the children, we can use it to find
            // which side of the screen the button is on.
            if let Some(mut camera_transform) = computed_target
                .camera()
                .and_then(|camera| camera_query.get_mut(camera).ok())
            {
                let angle = match direction {
                    Direction::Left => -0.1,
                    Direction::Right => 0.1,
                };
                camera_transform.rotate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::Y, angle));
            }
        }
    }
}



// Osef
// If windows resize => Then update little rectangle
pub fn resize_little_rectangle(
    windows: Query<&Window>,
    mut resize_events: EventReader<WindowResized>,
    mut query: Query<(&CameraPosition, &mut Camera), With<TopLeftCamera>>,
) {
    // We need to dynamically resize the camera's viewports whenever the window size changes
    // so then each camera always takes up half the screen.
    // A resize_event is sent when the window is first created, allowing us to reuse this system for initial setup.
    for resize_event in resize_events.read() {
        let window = windows.get(resize_event.window).unwrap();
        let size = window.physical_size()/4 ;
        // let size = UVec2::new(size.x/2 as u32, size.y as u32);

        for (camera_position, mut camera) in &mut query {
            camera.viewport = Some(Viewport {
                physical_position: camera_position.pos * size,
                physical_size: size,
                ..default()
            });
        }
    }
}
