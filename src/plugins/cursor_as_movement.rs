use std::time::Duration;

use bevy::prelude::*;


use super::player::controls_player_ball::{PlayerAnimationState, PlayerState};

pub struct RotateHeadPlugin;


impl Plugin for RotateHeadPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, animation_change)
            // .add_systems(Update, rstick_as_movement.run_if(in_state(super::app_state::AppState::Running)))
            ;
    }
}




fn animation_change(
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    mut pms: Query<&mut PlayerAnimationState, Changed<PlayerAnimationState>>,
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





// fn rstick_as_movement(
//     time: Res<Time>,
//     gamepads: Res<Gamepads>,
//     axes: Res<Axis<GamepadAxis>>,
//     mut query: Query<&mut Transform, With<Camera>>,
// ) {
//     let delta_time = time.delta_seconds();

//     for gamepad in gamepads.iter() {
//         let axis_lx = GamepadAxis {
//             gamepad,
//             axis_type: GamepadAxisType::RightStickX,
//         };
//         let axis_ly = GamepadAxis {
//             gamepad,
//             axis_type: GamepadAxisType::RightStickY,
//         };

//         if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
//             for mut transform in &mut query.iter_mut() {
//                 transform.rotate_y(-8.*x/360. * delta_time);
//                 transform.rotate_local_x(8.*y/360. * delta_time);
//             }
//         }

//     }
// }
