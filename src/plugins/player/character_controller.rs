use bevy::{
    input::{mouse::MouseMotion, InputSystem},
    prelude::*,
};
use bevy_rapier3d::{control::KinematicCharacterController, prelude::*};

const MOUSE_SENSITIVITY: f32 = 0.3;
const GROUND_TIMER: f32 = 0.5;
const MOVEMENT_SPEED: f32 = 8.0;
const JUMP_SPEED: f32 = 20.0;
const GRAVITY: f32 = -9.81;

pub struct CharacterControllerPlugin;
impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(ClearColor(Color::srgb(
                0x99 as f32 / 255.0,
                0xAA as f32 / 255.0,
                0xFF as f32 / 255.0,
            )))
            .init_resource::<MovementInput>()
            .init_resource::<LookInput>()
            .add_plugins((
                RapierPhysicsPlugin::<NoUserData>::default(),
                RapierDebugRenderPlugin::default(),
            ))
            .add_systems(Startup, (setup_player, setup_map))
            .add_systems(PreUpdate, handle_input.after(InputSystem))
            .add_systems(FixedUpdate, player_look)
            .add_systems(FixedUpdate, player_movement)
            ;
    }
}

pub fn setup_player(mut commands: Commands) {
    commands
        .spawn((
            Transform::from_xyz(0.0, 5.0, 0.0),
            Visibility::default(),
            Collider::round_cylinder(0.9, 0.3, 0.2),
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
        ))
        // .with_children(|b| {
        //     // FPS Camera
        //     b.spawn((Camera3d::default(), Transform::from_xyz(0.0, 0.2, -0.1)));
        // });
        ;
            // Spawn cam
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10.0, 5.0, 10.0),
        KinematicCharacterController {
            custom_mass: Some(0.),
            up: Vec3::Y,
            offset: CharacterLength::Absolute(0.01),
            slide: false,
            autostep: Some(CharacterAutostep {
                max_height: CharacterLength::Relative(0.3),
                min_width: CharacterLength::Relative(0.5),
                include_dynamic_bodies: false,
            }),
            apply_impulse_to_dynamic_bodies: false,
            snap_to_ground: None,
            ..default()
        },
    ));
}

fn setup_map(mut commands: Commands) {
    /*
     * Ground
     */
    let ground_size = 50.0;
    let ground_height = 0.1;

    commands.spawn((
        Transform::from_xyz(0.0, -ground_height, 0.0),
        Collider::cuboid(ground_size, ground_height, ground_size),
    ));
    /*
     * Stairs
     */
    let stair_len = 30;
    let stair_step = 0.2;
    for i in 1..=stair_len {
        let step = i as f32;
        let collider = Collider::cuboid(1.0, step * stair_step, 1.0);
        commands.spawn((
            Transform::from_xyz(40.0, step * stair_step, step * 2.0 - 20.0),
            collider.clone(),
        ));
        commands.spawn((
            Transform::from_xyz(-40.0, step * stair_step, step * -2.0 + 20.0),
            collider.clone(),
        ));
        commands.spawn((
            Transform::from_xyz(step * 2.0 - 20.0, step * stair_step, 40.0),
            collider.clone(),
        ));
        commands.spawn((
            Transform::from_xyz(step * -2.0 + 20.0, step * stair_step, -40.0),
            collider.clone(),
        ));
    }
}

/// Keyboard input vector
#[derive(Default, Resource, Deref, DerefMut)]
struct MovementInput(Vec3);

/// Mouse input vector
#[derive(Default, Resource, Deref, DerefMut)]
struct LookInput(Vec2);

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut movement: ResMut<MovementInput>,
    mut look: ResMut<LookInput>,
    mut mouse_events: EventReader<MouseMotion>,
) {
    if keyboard.pressed(KeyCode::KeyW) {
        movement.z -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        movement.z += 1.0
    }
    if keyboard.pressed(KeyCode::KeyA) {
        movement.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        movement.x += 1.0
    }
    **movement = movement.normalize_or_zero();
    if keyboard.pressed(KeyCode::ShiftLeft) {
        **movement *= 2.0;
    }
    if keyboard.pressed(KeyCode::Space) {
        movement.y = 1.0;
    }

    for event in mouse_events.read() {
        look.x -= event.delta.x * MOUSE_SENSITIVITY;
        look.y -= event.delta.y * MOUSE_SENSITIVITY;
        look.y = look.y.clamp(-89.9, 89.9); // Limit pitch
    }
}

fn player_movement(
    time: Res<Time>,
    mut input: ResMut<MovementInput>,
    mut player: Query<(
        &mut Transform,
        &mut KinematicCharacterController,
        Option<&KinematicCharacterControllerOutput>,
    ), Without<Camera3d>>,

    mut vertical_movement: Local<f32>,
    mut grounded_timer: Local<f32>,
) {
    let Ok((transform, mut controller, output)) = player.get_single_mut() else {
        return;
    };
    let delta_time = time.delta_secs();
    // Retrieve input
    let mut movement = Vec3::new(input.x, 0.0, input.z) * MOVEMENT_SPEED;
    let jump_speed = input.y * JUMP_SPEED;
    // Clear input
    **input = Vec3::ZERO;
    // Check physics ground check
    if output.map(|o| o.grounded).unwrap_or(false) {
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
    movement.y = *vertical_movement;
    *vertical_movement += GRAVITY * delta_time * controller.custom_mass.unwrap_or(1.0);
    controller.translation = Some(transform.rotation * (movement * delta_time));
}



fn player_look(
    mut player_query: Query<&mut Transform, (With<KinematicCharacterController>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
    mut windows: Query<&mut Window>,
) {

    let Ok(mut player_transform) = player_query.get_single_mut() else {
        println!("ERROR: in func player_look, player_query not single mut");
        return;
    };
    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        println!("ERROR: in func player_look, camera_query not single mut");
        return;
    };

    // let mut camera_transform = camera_query.single_mut();
    // let player_transform = player_query.single_mut();
    let delta_time = time.delta_secs();
    let mut window = windows.single_mut();
    let width_div_2 = window.resolution.width()/2.;
    let height_div_2 = window.resolution.height()/2.;
    // Games typically only have one window (the primary window)
    if let Some(cursor_position) = window.cursor_position() {
        let yaw = Quat::from_rotation_y(
            delta_time * 50.0 * (width_div_2 - cursor_position.x) / 360.,
        );
        camera_transform.rotate_around(player_transform.translation, yaw);
        let cam_local_x = camera_transform.right().as_vec3();
        let pitch = Quat::from_axis_angle(cam_local_x, 
            delta_time * 50.0 * (height_div_2 - cursor_position.y) / 360.
        );
        camera_transform.rotate_around(player_transform.translation, pitch);
        camera_transform.look_at(player_transform.translation, Vec3::Y);
    } else {
        println!("Cursor is not in the game window.");
    }
    window.set_cursor_position(Some((width_div_2, height_div_2).into()));
}


