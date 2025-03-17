
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{plugins::app_state::AppState, Player};

use super::spawn_player_ball::PreviousPosition;

// const SPEED: f32 = 10.0;
// const JUMP_FORCE: f32 = 10.0;
// const GRAVITY: f32 = -9.81;

pub struct ControlsPlayerBallPlugin;

impl Plugin for ControlsPlayerBallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(FixedUpdate, player_controller.run_if(in_state(AppState::Running)))
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
pub struct PlayerAnimationState {
    pub state: PlayerState,
}


fn player_controller(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut KinematicCharacterController, &mut PlayerAnimationState, &mut Transform, &mut PreviousPosition), (With<Player>, Without<Camera3d>)>, // TODO fix the super super super...
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    time: Res<Time>,
    mut windows: Query<&mut Window>,
) {
    let delta_time = time.delta_secs();
    // kcc stand for KinematicCharacterController
    let mut camera_transform = camera_query.single_mut();
    let (
        mut player_kcc,
        mut player_movement,
        mut player_transform,
        mut prev_pos
    ) = player_query.single_mut();

    println!("player_transform.translation.y: {:?}", player_transform.translation.y);

    // let vect_play_cam = camera_transform.translation - player_transform.translation;
    // let old_play_trans = player_transform.translation;
    // prev_pos.0 = player_transform.translation;

    let  left = keyboard_input.pressed(KeyCode::KeyA);
    let right = keyboard_input.pressed(KeyCode::KeyD);
    let  down = keyboard_input.pressed(KeyCode::KeyS);
    let    up = keyboard_input.pressed(KeyCode::KeyW);

    if left || right || down || up {
        // println!("run");
        if player_movement.state != PlayerState::Running {
            player_movement.state = PlayerState::Running;
        }
    } else {
        // println!("idls");
        if player_movement.state != PlayerState::Idle {
            player_movement.state = PlayerState::Idle;
        }
    }

    let mut direction = 
        (camera_transform.rotation * Vec3::X).normalize()*Vec3::new((right as i8 - left as i8) as f32, (right as i8 - left as i8) as f32, (right as i8 - left as i8) as f32)
        + (camera_transform.rotation * Vec3::Z).normalize()*Vec3::new((down as i8 - up as i8) as f32, (down as i8 - up as i8) as f32, (down as i8 - up as i8) as f32);

    // let direction = Vec2::new(direction.x, direction.z); // no need to direction.clamp_length_max(1.0) as it's already normalized
    

    direction.y = 0.;
    direction = direction.normalize_or_zero() * delta_time * 10.;

    
    // run fast
    if keyboard_input.pressed(KeyCode::Space) {
        direction.x *= 10.;
        direction.z *= 10.;
    }

    // We don't want the player return back to look 0 0 0 while no key pressed
    if direction != Vec3::ZERO {
        // Apply oriantation so player will look at dirrection he look at
        // So it will not get hurt hitting an obstacle <3
        player_transform.look_to(direction, Vec3::Y); // Do not want the Y component bc id down pressed, player will lay down on the floor
    }

    // Jump
    if keyboard_input.just_pressed(KeyCode::ShiftLeft) {
        player_movement.state = PlayerState::Jumping;
        direction.y = 10.;
    }


    // De-Jump
    if keyboard_input.pressed(KeyCode::KeyF) {
        direction.y = -1.;
    }



    // gravity lvl1
    direction.y += -5. * delta_time;
    
    // Apply Movement
    player_kcc.translation = Some(direction);


    // FUNCTION
    let new_position = player_transform.translation;
    let old_position = prev_pos.0;
    camera_transform.translation -= old_position - new_position;
    prev_pos.0 = new_position;






    // FUNCTION 2
    let mut window = windows.single_mut();

    let width_div_2 = window.resolution.width()/2.;
    let height_div_2 = window.resolution.height()/2.;

    // Games typically only have one window (the primary window)
    if let Some(cursor_position) = window.cursor_position() {


        let yaw = Quat::from_rotation_y(
            delta_time * 50.0 * (width_div_2 - cursor_position.x) / 360.,
        );
        camera_transform.rotate_around(player_transform.translation, yaw);
        // camera_transform.rotate_y( delta_time * 50. * (width_div_2  - cursor_position.x)/360.);


        let cam_local_x = camera_transform.right().as_vec3();
        let pitch = Quat::from_axis_angle(cam_local_x, 
            delta_time * 50.0 * (height_div_2 - cursor_position.y) / 360.
        );

        camera_transform.rotate_around(player_transform.translation, pitch);
        // camera_transform.rotate_local_x(delta_time * 50. * (height_div_2 - cursor_position.y)/360.);

        // camera_transform.rotate_local_x( pitch);

        
        camera_transform.look_at(player_transform.translation, Vec3::Y);
    } else {
        println!("Cursor is not in the game window.");
    }

    window.set_cursor_position(Some((width_div_2, height_div_2).into()));
    // window.set_physical_cursor_position(Some((0.0, 0.0).into()));



}








