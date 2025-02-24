use bevy::prelude::*;

const SPEED: f32 = 10.0;
const JUMP_FORCE: f32 = 10.0;
const GRAVITY: f32 = -9.81;

pub struct ControlsPlayerBallPlugin;

impl Plugin for ControlsPlayerBallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, player_controller)
            ;
    }
}




fn player_controller(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<super::super::super::Player>>, // TODO fix the super super super...
    time: Res<Time>,
) {
    let mut trans_rot= Quat::from_xyzw(0., 0., 0., 0.);
    for transform in &mut query.iter_mut() {
        trans_rot = transform.rotation;
        // println!("trans_rot: {:?}", trans_rot)
    }

    let  left = keyboard_input.pressed(KeyCode::KeyA);
    let right = keyboard_input.pressed(KeyCode::KeyD);
    let  down = keyboard_input.pressed(KeyCode::KeyS);
    let    up = keyboard_input.pressed(KeyCode::KeyW);

    let direction = 
          (trans_rot * Vec3::X).normalize()*Vec3::new((right as i8 - left as i8) as f32, (right as i8 - left as i8) as f32, (right as i8 - left as i8) as f32)
        + (trans_rot * Vec3::Z).normalize()*Vec3::new((down as i8 - up as i8) as f32, (down as i8 - up as i8) as f32, (down as i8 - up as i8) as f32);

    let direction = Vec2::new(direction.x, direction.z); // no need to direction.clamp_length_max(1.0) as it's already normalized

    let delta_time = time.delta_secs();

    // Apply Movement
    if direction != Vec2::ZERO {
        for mut transform in &mut query.iter_mut() {
            transform.translation.x += direction.x * delta_time * 10.;
            transform.translation.z += direction.y * delta_time * 10.;
        }
    }

    // Jump
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        for mut transform in &mut query.iter_mut() {
            transform.translation.y += 0.2;
        }
    }

    // De-Jump
    if keyboard_input.pressed(KeyCode::KeyF) {
        for mut transform in &mut query.iter_mut() {
            transform.translation.y -= 0.2;
        }
    }
}