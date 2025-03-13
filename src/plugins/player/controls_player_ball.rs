use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::Player;

// const SPEED: f32 = 10.0;
// const JUMP_FORCE: f32 = 10.0;
// const GRAVITY: f32 = -9.81;

pub struct ControlsPlayerBallPlugin;

impl Plugin for ControlsPlayerBallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, player_controller)
            .add_systems(Update, change_detection)
            ;
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerState {
    Idle,
    Running,
    Jumping,
    Sleepy,
}
#[derive(Component)]
pub struct PlayerMovement {
    pub state: PlayerState,
}











fn player_controller(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut KinematicCharacterController, &mut PlayerMovement, &Transform), (With<Player>, Without<Camera3d>)>, // TODO fix the super super super...
    mut camera_query: Query<(&mut Transform), (With<Camera3d>, Without<Player>)>,
    time: Res<Time>,
) {
    // kcc stand for KinematicCharacterController
    let mut camera_transform = camera_query.single_mut();
    let (mut player_kcc, mut pm, player_transform) = player_query.single_mut();


        
    let trans_rot= camera_transform.rotation.clone();

    let  left = keyboard_input.pressed(KeyCode::KeyA);
    let right = keyboard_input.pressed(KeyCode::KeyD);
    let  down = keyboard_input.pressed(KeyCode::KeyS);
    let    up = keyboard_input.pressed(KeyCode::KeyW);

    if left || right || down || up {
        // println!("run");
        if pm.state != PlayerState::Running {
            pm.state = PlayerState::Running;
        }
    } else {
        // println!("idls");
        if pm.state != PlayerState::Idle {
            pm.state = PlayerState::Idle;
        }
    }

    let mut direction = 
        (trans_rot * Vec3::X).normalize()*Vec3::new((right as i8 - left as i8) as f32, (right as i8 - left as i8) as f32, (right as i8 - left as i8) as f32)
        + (trans_rot * Vec3::Z).normalize()*Vec3::new((down as i8 - up as i8) as f32, (down as i8 - up as i8) as f32, (down as i8 - up as i8) as f32);

    // let direction = Vec2::new(direction.x, direction.z); // no need to direction.clamp_length_max(1.0) as it's already normalized
    

    direction.y = 0.;
    direction = direction.normalize_or_zero() * time.delta_secs() * 10.;

    // Jump
    if keyboard_input.just_pressed(KeyCode::ShiftLeft) {
        pm.state = PlayerState::Jumping;
        direction.y = 10.;
    }
    if keyboard_input.pressed(KeyCode::Space) {
        direction.x *= 10.;
        direction.z *= 10.;
    }

    // De-Jump
    if keyboard_input.pressed(KeyCode::KeyF) {
        direction.y = -1.;
    }

    // Apply Movement
    direction.y += -10. * time.delta_secs();
    
    player_kcc.translation = Some(direction);


    //
    // Fix cam position
    let dir = camera_transform.forward();
    let player_position = player_transform.translation;
    let camera_position = camera_transform.translation;
    let distance_play_cam = player_position.distance(camera_position);
    let distance_i_want_between_the_cam_and_the_player = 10.;
    let delta_distance = distance_i_want_between_the_cam_and_the_player - distance_play_cam;
    // let camera_direction = (camera_rotation * Vec3::X).normalize() + (camera_rotation * Vec3::Z).normalize();

    if distance_play_cam > distance_i_want_between_the_cam_and_the_player {
        // camera_transform.translation += camera_direction * (distance_play_cam - 50.);
        camera_transform.translation += dir * (distance_play_cam - distance_i_want_between_the_cam_and_the_player);
        
    } else if distance_play_cam < distance_i_want_between_the_cam_and_the_player {
        camera_transform.translation -= dir * (delta_distance);
        // let target_position = camera_transform.translation - dir * (delta_distance);
        // camera_transform.translation = camera_transform.translation.lerp(target_position, 0.1); // Smooth follow

    }

}



fn change_detection(
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    mut pms: Query<&mut PlayerMovement, Changed<PlayerMovement>>,
    animations: Res<crate::plugins::animation::Animations>,
) {
    for pm in &mut pms {
        // println!("{:?}", pm.state);
        // change animation
        for (mut player, mut transitions) in &mut animation_players {
            match pm.state {
                PlayerState::Running => {
                    transitions
                    .play(
                        &mut player,
                        animations.animations[3], // Running animation
                        Duration::from_millis(250),
                    )
                    .repeat();
                },
                PlayerState::Jumping => {
                    transitions
                    .play(
                        &mut player,
                        animations.animations[1],
                        Duration::from_millis(250),
                    )
                    .repeat();
                },
                _ => {
                    transitions
                    .play(
                        &mut player,
                        animations.animations[0],
                        Duration::from_millis(250),
                    )
                    .repeat();
                },
            }
        }
    }
}
