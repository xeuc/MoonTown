use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// const SPEED: f32 = 10.0;
// const JUMP_FORCE: f32 = 10.0;
// const GRAVITY: f32 = -9.81;

pub struct ControlsPlayerBallPlugin;

impl Plugin for ControlsPlayerBallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, player_controller)
            .add_systems(Update, change_detection)
            // .add_systems(Update, gravity)
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
    mut query: Query<(&mut Transform, &mut KinematicCharacterController, &mut PlayerMovement), With<super::super::super::Player>>, // TODO fix the super super super...
    time: Res<Time>,

) {

    for (transform, mut kinematic_character_controller, mut pm) in &mut query.iter_mut() {
        
                
        let trans_rot= transform.rotation;

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
        
        kinematic_character_controller.translation = Some(direction);

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
        if pm.state == PlayerState::Running {
            transitions
                .play(
                    &mut player,
                    animations.animations[3], // Running animation
                    Duration::from_millis(250),
                )
                .repeat();
        } else {
            transitions
                .play(
                    &mut player,
                    animations.animations[0], // idle animation
                    Duration::from_millis(250),
                )
                .repeat();
        }
       
    }



}

}
