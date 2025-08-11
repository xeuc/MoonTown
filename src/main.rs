use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, input::{mouse::MouseMotion, InputSystem}, prelude::*
};
use bevy_rapier3d::{control::KinematicCharacterController, prelude::*};
use bevy::{
    render::camera::Viewport, window::WindowResized,
};


mod spawners;
use spawners::{
    CameraPosition, RotateCamera, TopLeftCamera, Direction,
};

const MOUSE_SENSITIVITY: f32 = 0.3;
const GROUND_TIMER: f32 = 0.5;
const MOVEMENT_SPEED: f32 = 8.0;
const JUMP_SPEED: f32 = 20.0;
const GRAVITY: f32 = -9.81;

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
            
            // The other diagnostics plugins can still be used without this if you want to use them in an ingame overlay for example.
            LogDiagnosticsPlugin::default(),
            // Adds frame time, FPS and frame count diagnostics.
            FrameTimeDiagnosticsPlugin::default(),
            // Adds an entity count diagnostic.
            bevy::diagnostic::EntityCountDiagnosticsPlugin,
            // Adds cpu and memory usage diagnostics for systems and the entire game process.
            bevy::diagnostic::SystemInformationDiagnosticsPlugin,
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
        .add_systems(FixedUpdate, translate_player)
        // .add_systems(FixedUpdate, translate_cam.after(translate_player))
        .add_systems(Update, (
            rotate_player_and_cam,
            rotate_button_behavior,
            resize_little_rectangle,
        ))

        .run();
}


/// Keyboard input vector
#[derive(Default, Resource, Deref, DerefMut)]
struct MovementInput(Vec3);

/// Mouse input vector
#[derive(Default, Resource, Deref, DerefMut)]
struct LookInput(Vec2);


















// keyboard  => movement_input: ResMut<MovementInput> => for translate_player_and_cam()
// mouse pos => look_input:     ResMut<LookInput>     => for rotate_player_and_cam()
fn handle_input(
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
// no cam translate anymore
// movement_input (keyboard)
fn translate_player(
    time: Res<Time>,
    mut movement_input: ResMut<MovementInput>,
    mut player: Query<(
        &mut Transform,
        &mut KinematicCharacterController,
        Option<&KinematicCharacterControllerOutput>,
    ), Without<Camera>>,
    mut vertical_movement: Local<f32>,
    mut grounded_timer: Local<f32>,
) {
    // let mut counter = 0; // first is cam, second is player
    for (player_transform, mut player_controller, player_output) in player.iter_mut() {
        // print!("------------------------------------------------\n");
        // counter += 1;
        // print!("counter: {:?}\n", counter);
        // print!("player_transform: {:?}\n", player_transform);
        // print!("player_controller: {:?}\n", player_controller);
        // print!("player_output: {:?}\n", player_output);

        // print!("movement_input: {:?}\n", movement_input[0]);
        let delta_time = time.delta_secs();
        // Retrieve input
        let mut player_movement = Vec3::new(movement_input.x, 0.0, movement_input.z) * MOVEMENT_SPEED;
        // print!("player_movement: {:?}\n", player_movement);
        let jump_speed = movement_input.y * JUMP_SPEED;
        // Clear input
        // if counter == 2 {
            **movement_input = Vec3::ZERO;
        // }
        // Check physics ground check
        if player_output.map(|o| o.grounded).unwrap_or(false) {
            *grounded_timer = GROUND_TIMER;
            *vertical_movement = 0.0;
        }
        // If we are grounded we can jump
        if *grounded_timer > 0.0 {
            *grounded_timer -= delta_time;
            // If we jump we clear the grounded tolerance
            if jump_speed > 0.0 {
                *vertical_movement = jump_speed;
                *grounded_timer = 0.0;
            }
        }
        player_movement.y = *vertical_movement;
        *vertical_movement += GRAVITY * delta_time * player_controller.custom_mass.unwrap_or(1.0);
        player_controller.translation = Some(player_transform.rotation * (player_movement * delta_time));

    }

}

// ACTUALLY TRANSLATE THE PLAYER AND THE CAMERA (6 directions)
// no cam translate anymore
// movement_input (keyboard)
fn _translate_cam(
    time: Res<Time>,
    mut movement_input: ResMut<MovementInput>,
    mut player: Query<(
        &mut Transform,
        &mut KinematicCharacterController,
        Option<&KinematicCharacterControllerOutput>,
    ), (With<Camera3d>, Without<TopLeftCamera>)>,
    mut vertical_movement: Local<f32>,
    mut grounded_timer: Local<f32>,
) {
    // let mut counter = 0; // first is cam, second is player
    for (player_transform, mut player_controller, player_output) in player.iter_mut() {
        // print!("------------------------------------------------\n");
        // counter += 1;
        // print!("counter: {:?}\n", counter);
        // print!("player_transform: {:?}\n", player_transform);
        // print!("player_controller: {:?}\n", player_controller);
        // print!("player_output: {:?}\n", player_output);

        // print!("movement_input: {:?}\n", movement_input[0]);
        let delta_time = time.delta_secs();
        // Retrieve input
        let mut player_movement = Vec3::new(movement_input.x, 0.0, movement_input.z) * MOVEMENT_SPEED;
        // print!("player_movement: {:?}\n", player_movement);
        let jump_speed = movement_input.y * JUMP_SPEED;
        // Clear input
        // if counter == 2 {
            **movement_input = Vec3::ZERO;
        // }
        // Check physics ground check
        if player_output.map(|o| o.grounded).unwrap_or(false) {
            *grounded_timer = GROUND_TIMER;
            *vertical_movement = 0.0;
        }
        // If we are grounded we can jump
        if *grounded_timer > 0.0 {
            *grounded_timer -= delta_time;
            // If we jump we clear the grounded tolerance
            if jump_speed > 0.0 {
                *vertical_movement = jump_speed;
                *grounded_timer = 0.0;
            }
        }
        player_movement.y = *vertical_movement;
        *vertical_movement += GRAVITY * delta_time * player_controller.custom_mass.unwrap_or(1.0);
        player_controller.translation = Some(player_transform.rotation * (player_movement * delta_time));

    }

}





















// ACTUALLY ROTATE THE PLAYER (left and right) AND CAMERA (all 4 directions)
// btw should not code for Cam as Cam = child(Player)
//   => Actually no, it's for the cam to say "yes" (up and down)
// look_input (mouse)
fn rotate_player_and_cam(
    mut player: Query<&mut Transform, (With<KinematicCharacterController>, Without<Camera>)>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<TopLeftCamera>)>,
    look_input: Res<LookInput>,
) {
    let Ok(mut player_transform) = player.single_mut() else { return; };
    player_transform.rotation = Quat::from_axis_angle(Vec3::Y, look_input.x.to_radians());
    let Ok(mut camera_transform) = camera.single_mut() else { return;};
    // camera_transform.rotation = Quat::from_axis_angle(Vec3::X, look_input.y.to_radians());
    camera_transform.rotation = Quat::from_axis_angle(Vec3::Y, look_input.x.to_radians()) *
        Quat::from_axis_angle(Vec3::X, look_input.y.to_radians());
}




















// Osef
// Rotate button behavior
fn rotate_button_behavior(
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
fn resize_little_rectangle(
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