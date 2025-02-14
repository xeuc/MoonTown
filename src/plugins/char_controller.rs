use std::f32::consts::PI;
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

/// A marker component indicating that an entity is using a character controller.
#[derive(Component)]
pub struct CharacterController;

/// The acceleration used for character movement.
#[derive(Component)]
pub struct MovementAcceleration(f32);

/// The damping factor used for slowing down movement.
#[derive(Component)]
pub struct MovementDampingFactor(f32);

/// The strength of a jump.
#[derive(Component)]
pub struct JumpImpulse(f32);

/// The maximum angle a slope can have for a character controller
/// to be able to climb and jump. If the slope is steeper than this angle,
/// the character will slide down.
#[derive(Component)]
pub struct MaxSlopeAngle(f32);

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    movement: MovementBundle,
}

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
    jump_impulse: JumpImpulse,
    max_slope_angle: MaxSlopeAngle,
}

impl MovementBundle {
    pub const fn new(
        acceleration: f32,
        damping: f32,
        jump_impulse: f32,
        max_slope_angle: f32,
    ) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: MovementDampingFactor(damping),
            jump_impulse: JumpImpulse(jump_impulse),
            max_slope_angle: MaxSlopeAngle(max_slope_angle),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0, 0.9, 7.0, PI * 0.45)
    }
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


