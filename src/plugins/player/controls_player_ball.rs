use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::Player;

use super::spawn_player_ball::PreviousPosition;

// const SPEED: f32 = 10.0;
// const JUMP_FORCE: f32 = 10.0;
// const GRAVITY: f32 = -9.81;

pub struct ControlsPlayerBallPlugin;

impl Plugin for ControlsPlayerBallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, player_controller)
            .add_systems(Update, change_detection)
            .add_systems(Update, apply_movement)
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
    mut player_query: Query<(&mut KinematicCharacterController, &mut PlayerMovement, &mut Transform, &mut PreviousPosition), (With<Player>, Without<Camera3d>)>, // TODO fix the super super super...
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    time: Res<Time>,
) {
    // kcc stand for KinematicCharacterController
    let mut camera_transform = camera_query.single_mut();
    let (mut player_kcc, mut pm, mut player_transform, mut pp) = player_query.single_mut();

    let vect_play_cam = camera_transform.translation - player_transform.translation;
    let old_play_trans = player_transform.translation;
    pp.0 = player_transform.translation;

    let trans_rot= camera_transform.rotation;

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

    // Apply oriantation so player will look at dirrection he look at
    // So it will not get hurt hitting an obstacle <3
    player_transform.look_to(direction, Vec3::Y);

    // gravity lvl1
    direction.y += -10. * time.delta_secs();
    
    // Apply Movement
    player_kcc.translation = Some(direction);


    // don't work
    // if old_play_trans != player_transform.translation {
    //     println!("different")
    // } else {
    //     println!("same")
    // }
    // camera_transform.translation = vect_play_cam + player_transform.translation;



    // // Fix cam position
    // let dir = camera_transform.forward();
    // let player_position = player_transform.translation;
    // let camera_position = camera_transform.translation;
    // let distance_play_cam = player_position.distance(camera_position);
    // let distance_i_want_between_the_cam_and_the_player = 10.;
    // let delta_distance = distance_i_want_between_the_cam_and_the_player - distance_play_cam;
    // // let camera_direction = (camera_rotation * Vec3::X).normalize() + (camera_rotation * Vec3::Z).normalize();

    // if distance_play_cam > distance_i_want_between_the_cam_and_the_player {
    //     // camera_transform.translation += camera_direction * (distance_play_cam - 50.);
    //     camera_transform.translation += dir * (distance_play_cam - distance_i_want_between_the_cam_and_the_player);
        
    // } else if distance_play_cam < distance_i_want_between_the_cam_and_the_player {
    //     camera_transform.translation -= dir * (delta_distance);
    //     // let target_position = camera_transform.translation - dir * (delta_distance);
    //     // camera_transform.translation = camera_transform.translation.lerp(target_position, 0.1); // Smooth follow
    // }

    // Actually cam is programmed to keep the same distance player-camera
    // But this behavior is wrong bc it generate a bug:
    // when cam look at handsome side of player + user touch "d" button,
    // player will turn around camera (which will remain fix)
    // Instead, the camera should follow the player

    // + The player shake for no reason

    // + I want to use lerp()

}


// fn syn_cam_and_player(
//     old_player_pos: pos,
//     mut player_query: Query<(&mut KinematicCharacterController, &mut PlayerMovement, &mut Transform), (With<Player>, Without<Camera3d>)>, // TODO fix the super super super...
//     mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
// ) {
//     let mut camera_transform = camera_query.single_mut();
//     let (mut player_kcc, mut pm, mut player_transform) = player_query.single_mut();

//     camera_transform.translation += player_transform.translation - old_player_pos;
// }




fn apply_movement(
    mut query: Query<(&mut Transform, &PreviousPosition), With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok((mut player_transform, prev_pos)) = query.get_single_mut() {
        let new_position = player_transform.translation;
        let old_position = prev_pos.0;

        // Appliquer ce delta à la caméra
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation -= old_position - new_position;
        }
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
