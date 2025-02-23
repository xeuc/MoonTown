use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const SPEED: f32 = 10.0;
const JUMP_FORCE: f32 = 10.0;
const GRAVITY: f32 = -9.81;

pub struct ControlsPlayerBallPlugin;

impl Plugin for ControlsPlayerBallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, player_movement)
            ;
    }
}


fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut controllers: Query<&mut KinematicCharacterController>,
) {
    for mut controller in controllers.iter_mut() {
        let mut movement = Vec3::ZERO;
        let mut curent_speed = SPEED;

        if keyboard_input.pressed(KeyCode::Numpad5) {movement.z -= 1.0;}
        if keyboard_input.pressed(KeyCode::Numpad2) {movement.z += 1.0;}
        if keyboard_input.pressed(KeyCode::Numpad1) {movement.x -= 1.0;}
        if keyboard_input.pressed(KeyCode::Numpad3) {movement.x += 1.0;}
        if keyboard_input.pressed(KeyCode::Numpad4) {curent_speed = 1.;}
        if keyboard_input.pressed(KeyCode::Numpad6) {curent_speed = 50.;}

        movement = movement.normalize_or_zero() * curent_speed * time.delta_secs();

        if keyboard_input.just_pressed(KeyCode::Numpad0) {movement.y += JUMP_FORCE;}

        movement.y += GRAVITY * time.delta_secs();
        controller.translation = Some(movement);
    }
}