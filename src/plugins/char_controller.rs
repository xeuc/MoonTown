use bevy::prelude::*;

use super::app_state::AppState;

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update,player_controller.run_if(in_state(AppState::Running)));
    }
}


// Not sending events be I don't want to decorrelate user input from motion processing, yet
fn player_controller(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
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

    let delta_time = time.delta_secs();

    // Aply Movement
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


