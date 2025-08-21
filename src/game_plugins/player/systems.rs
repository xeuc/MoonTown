use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy::{prelude::*, pbr::CascadeShadowConfigBuilder};
use bevy_rapier3d::{control::KinematicCharacterController, prelude::*};
use std::f32::consts::PI;



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











/// ******************************************************************************
/// ***  PLAYER + CAM0 ***********************************************************
/// ******************************************************************************

// ---ANCHOR---
//    |     |
//    V     V
// PLAYER CAMERA

//         | Need to Translate?                  | Need to Rotate?               |
// Player  | Translate the anchor                | Rotate the Player             |
// Camera  | Get closer/away of player, move Cam | Rotate Camera around (Player) |
// Note: To get the RIGHT position of the player, and avoid jitter and shaking from the player:
//       PLEASE QUERRY THE PLAYER TRANSLATE. Not the anchor translate,
//       not the anchor global transform, not the kcc, not the kcc output, bc they are all broken.

// Setup player entity and its integrated camera
pub fn setup_player_camera_integrated(mut commands: Commands) {
    // Anchor
    commands.spawn((
        Anchor,
        Transform::from_xyz(0.0, 5.0, 0.0),
        Visibility::default(),
        Collider::round_cylinder(0.9, 0.3, 0.2),
        Name::new("Anchor"),
        KinematicCharacterController {
            custom_mass: Some(5.0),
            up: Vec3::Y,
            offset: CharacterLength::Absolute(0.01),
            slide: true,
            autostep: Some(CharacterAutostep {
                max_height: CharacterLength::Relative(0.3),
                min_width: CharacterLength::Relative(0.5),
                include_dynamic_bodies: false,
            }),
            // Don’t allow climbing slopes larger than 45 degrees.
            max_slope_climb_angle: 45.0_f32.to_radians(),
            // Automatically slide down on slopes smaller than 30 degrees.
            min_slope_slide_angle: 30.0_f32.to_radians(),
            apply_impulse_to_dynamic_bodies: true,
            snap_to_ground: None,
            ..default()
        },
        Children::spawn((
            Spawn((
                Name::new("Player"),
                Player,
                Visibility::default(),
                Transform::from_xyz(0.0, 0.0, 0.0),
                
            )),
            Spawn((
                Name::new("PlayerCamera"),
                PlayerCamera,
                Camera3d::default(), 
                Transform::from_xyz(0.0, 1.0, 5.0).looking_at((0.0, 5.0, 0.0).into(), Vec3::Y),
                Camera {
                    // Renders cameras with different priorities to prevent ambiguities
                    order: 0,
                    ..default()
                },
            )),
        )),
    ));
}












/// ******************************************************************************
/// ***  MAP  ********************************************************************
/// ******************************************************************************

pub fn setup_maps_elements(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    let ground_size = 50.0;
    let ground_height = 0.1;
    commands.spawn((
        Name::new("Ground"),
        Mesh3d(meshes.add(Cuboid::new(2.0*ground_size, 2.0*ground_height, 2.0*ground_size))),
        // MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, -ground_height, 0.0),
        Collider::cuboid(ground_size, ground_height, ground_size),
    ));



    // Stairs-Wall
    let stair_len = 30;
    let stair_step = 0.2;
    for i in 1..=stair_len {
        let step = i as f32;
        let collider = Collider::cuboid(1.0, step * stair_step, 1.0);
        let cuboid = Mesh3d(meshes.add(Cuboid::new(2.0*1.0, 2.0*step * stair_step, 2.0*1.0)));
        let material= MeshMaterial3d(materials.add(Color::srgb(153.0/255.0, 90.0/255.0, 50.0/255.0)));
        commands.spawn((
            Name::new("Stair4"),
            Transform::from_xyz(40.0, step * stair_step, step * 2.0 - 20.0),
            cuboid.clone(),
            material.clone(),
            collider.clone(),
        ));
        commands.spawn((
            Name::new("Stair3"),
            Transform::from_xyz(-40.0, step * stair_step, step * -2.0 + 20.0),
            cuboid.clone(),
            material.clone(),
            collider.clone(),
        ));
        commands.spawn((
            Name::new("Stair2"),
            Transform::from_xyz(step * 2.0 - 20.0, step * stair_step, 40.0),
            cuboid.clone(),
            material.clone(),
            collider.clone(),
        ));
        commands.spawn((
            Name::new("Stair1"),
            Transform::from_xyz(step * -2.0 + 20.0, step * stair_step, -40.0),
            cuboid.clone(),
            material.clone(),
            collider.clone(),
        ));
    }

    // Light
    commands.spawn((
        Name::new("Light"),
        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        CascadeShadowConfigBuilder {
            num_cascades: 2,
            first_cascade_far_bound: 200.0,
            maximum_distance: 280.0,
            ..default()
        }
        .build(),
    ));
}









/// ******************************************************************************
/// ***  UI = CAM2 in top right corner  ******************************************
/// ******************************************************************************


// set up the UI GFY
pub fn setup_ui(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Cameras and their dedicated UI
    let camera = commands
        .spawn((
            Camera3d::default(),
            Transform::from_translation(Vec3::new(0.0, 3.0, -30.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            Camera {
                // Renders cameras with different priorities to prevent ambiguities
                order: 1 as isize,
                ..default()
            },
            TopLeftCamera,
            CameraPosition {
                pos: UVec2::new((0 % 2) as u32, (0 / 2) as u32),
            },
        ))
        .id();

    // Set up UI (Little rectangle at the top left corner)
    // Wrapper of both button PLUS TEXT that have the same portion as it's parent
    // dynamicly set IG 
    // It's invisible and just help to set the position of the 2 buttons
    commands
        .spawn((
            UiTargetCamera(camera),
            // if you don't put the node element,
            // WARN bevy_ecs::hierarchy: warning[B0004]: 
            // Entity 134v1 with the GlobalTransform component has a parent without GlobalTransform
            // WARN bevy_ecs::hierarchy: warning[B0004]: 
            // Entity 134v1 with the InheritedVisibility component has a parent without InheritedVisibility.
            Node {
                // 100% is 100% of the rectangle
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Player 1"),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(12.),
                    left: Val::Px(12.),
                    // height and width are dynamicly set by
                    // set_camera_viewports function
                    ..default()
                },
            ));
            buttons_panel_spawner(parent);
        });

    // Wrapper of both button that have the same portion as it's parent
    // It's invisible and just help to set the position of the 2 buttons
    fn buttons_panel_spawner(parent: &mut ChildSpawnerCommands) {
        parent
            .spawn(Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.)),
                ..default()
            })
            .with_children(|parent| {
                rotate_button_spawner(parent, "<", Direction::Left);
                rotate_button_spawner(parent, ">", Direction::Right);
            });
    }

    fn rotate_button_spawner(parent: &mut ChildSpawnerCommands, caption: &str, direction: Direction) {
        parent
            .spawn((
                RotateCamera(direction),
                Button,
                Node {
                    width: Val::Px(40.),
                    height: Val::Px(40.),
                    border: UiRect::all(Val::Px(2.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(Color::WHITE),
                BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
            ))
            .with_children(|parent| {
                parent.spawn(Text::new(caption));
            });
    }
}












































































// ******************************************************************************
// UPDATES
// ******************************************************************************


use bevy::{
    input::mouse::MouseMotion
};
use bevy::{
    render::camera::Viewport, window::WindowResized,
};

use crate::game_plugins::player::components::*;





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
