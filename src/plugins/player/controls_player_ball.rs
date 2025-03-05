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
            // .add_systems(Update, gravity)
            ;
    }
}




fn player_controller(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut KinematicCharacterController), With<super::super::super::Player>>, // TODO fix the super super super...
    time: Res<Time>,
) {
    for (transform, mut kinematic_character_controller) in &mut query.iter_mut() {
        let trans_rot= transform.rotation;

        let  left = keyboard_input.pressed(KeyCode::KeyA);
        let right = keyboard_input.pressed(KeyCode::KeyD);
        let  down = keyboard_input.pressed(KeyCode::KeyS);
        let    up = keyboard_input.pressed(KeyCode::KeyW);

        let mut direction = 
            (trans_rot * Vec3::X).normalize()*Vec3::new((right as i8 - left as i8) as f32, (right as i8 - left as i8) as f32, (right as i8 - left as i8) as f32)
            + (trans_rot * Vec3::Z).normalize()*Vec3::new((down as i8 - up as i8) as f32, (down as i8 - up as i8) as f32, (down as i8 - up as i8) as f32);

        // let direction = Vec2::new(direction.x, direction.z); // no need to direction.clamp_length_max(1.0) as it's already normalized
        

        direction.y = 0.;
        direction = direction.normalize_or_zero() * time.delta_secs() * 50.;

        // Jump
        if keyboard_input.just_pressed(KeyCode::ShiftLeft) {
            direction.y = 0.25;
        }
        if keyboard_input.just_pressed(KeyCode::Space) {
            direction.y = 10.;
        }

        // De-Jump
        if keyboard_input.pressed(KeyCode::KeyF) {
            direction.y = -1.;
        }

        // Apply Movement
        direction.y += -0.5 * time.delta_secs();
        
        kinematic_character_controller.translation = Some(direction);

    }

}




// fn gravity(
//     mut query: Query<&mut Transform, With<Collider>>, // TODO fix the super super super...
// ) {
//     // Gravity
//     for mut transform in &mut query.iter_mut() {
//         transform.translation.y -= 0.01;
//     }
// }