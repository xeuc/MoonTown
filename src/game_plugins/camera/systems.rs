
use bevy::prelude::*;

use crate::game_plugins::shared::components::*;

// take the input mouse from look_input, and use it to rotate the camera around the player
pub fn rotate_cam_from_look_input(
    mut camera: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    mut player: Query<&mut Transform, (With<Player>, Without<PlayerCamera>)>,
    mut look_input: ResMut<LookInput>,
) {
    let Ok(mut camera_transform) = camera.single_mut() else { return;};
    // I use player instead of anchor to avoid problem of jitter + problem of player shaking along Y ¯\_(ツ)_/¯
    let Ok(player_transform) = player.single_mut() else { return;};

    let target = player_transform.translation;
    let cam_local_x = camera_transform.right().as_vec3();
    let pitch = Quat::from_axis_angle(cam_local_x, look_input.y.to_radians());
    let yaw = Quat::from_rotation_y(look_input.x.to_radians());
    let rotation = yaw * pitch;
    camera_transform.rotate_around(target, rotation);
    camera_transform.look_at(target, Vec3::Y);

    // Reset look input after applying it
    **look_input = Vec2::ZERO;
}





