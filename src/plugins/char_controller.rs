use bevy::prelude::*;

use super::app_state::AppState;

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementAction>().add_systems(
            Update,
            (
                keyboard_input,
                movement,
            ).run_if(in_state(AppState::Running))
        );
    }
}

/// An event sent for a movement input action.
#[derive(Event)]
pub enum MovementAction {
    Move(Vec2),
    Jump,
    DeJump,
}




/// Sends [`MovementAction`] events based on keyboard input.
fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let mut trans_rot= Quat::from_xyzw(0., 0., 0., 0.);
    for transform in &mut query.iter_mut() {
        trans_rot = transform.rotation;
    }

    let  left = keyboard_input.pressed(KeyCode::KeyA);
    let right = keyboard_input.pressed(KeyCode::KeyD);
    let  down = keyboard_input.pressed(KeyCode::KeyS);
    let    up = keyboard_input.pressed(KeyCode::KeyW);

    let direction = 
          (trans_rot * Vec3::X).normalize()*Vec3::new((right as i8 - left as i8) as f32, (right as i8 - left as i8) as f32, (right as i8 - left as i8) as f32)
        + (trans_rot * Vec3::Z).normalize()*Vec3::new((down as i8 - up as i8) as f32, (down as i8 - up as i8) as f32, (down as i8 - up as i8) as f32);

    let direction = Vec2::new(direction.x, direction.z); // no need to direction.clamp_length_max(1.0) as it's already normalized


    
    if direction != Vec2::ZERO {
        movement_event_writer.send(MovementAction::Move(direction));
    }

    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        movement_event_writer.send(MovementAction::Jump);
    }

    if keyboard_input.pressed(KeyCode::KeyF) {
        movement_event_writer.send(MovementAction::DeJump);
    }
}


/// Responds to [`MovementAction`] events and moves character controllers accordingly.
fn movement(
    time: Res<Time>,
    mut movement_event_reader: EventReader<MovementAction>,
    mut transform_of_player: Query<&mut Transform, With<Camera>>,
) {
    let delta_time = time.delta_secs();

    for event in movement_event_reader.read() {
        match event {
            MovementAction::Move(direction) => {
                for mut transform in &mut transform_of_player.iter_mut() {
                    transform.translation.x += direction.x * delta_time * 500.;
                    transform.translation.z += direction.y * delta_time * 500.;
                }
            }
            MovementAction::Jump => {
                for mut transform in &mut transform_of_player.iter_mut() {
                    transform.translation.y += 1.;
                }
            }
            MovementAction::DeJump => {
                for mut transform in &mut transform_of_player.iter_mut() {
                    transform.translation.y -= 1.;
                }
            }
        }
    }
}


