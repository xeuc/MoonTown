use bevy::prelude::*;
use bevy_rapier3d::{control::KinematicCharacterController, prelude::*};
use crate::game_plugins::shared::components::*;



//   _________       __                
//  /   _____/ _____/  |_ __ ________  
//  \_____  \_/ __ \   __\  |  \____ \ 
//  /        \  ___/|  | |  |  /  |_> >
// /_______  /\___  >__| |____/|   __/ 
//         \/     \/           |__|    












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











































//  ____ ___            .___       __ ___________
// |    |   \______   __| _/____ _/  |\_   _____/
// |    |   /\____ \ / __ |\__  \\   __\    __)_ 
// |    |  / |  |_> > /_/ | / __ \|  | |        \
// |______/  |   __/\____ |(____  /__|/_______  /
//           |__|        \/     \/            \/ 













































// ******************************************************************************
// UPDATES
// ******************************************************************************


use crate::game_plugins::player::components::*;









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




