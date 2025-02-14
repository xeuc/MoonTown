use bevy::prelude::*;

pub struct RotateHeadPlugin;


impl Plugin for RotateHeadPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, cursor_as_movement.run_if(in_state(super::app_state::AppState::Running)))
            // .add_systems(Update, rstick_as_movement.run_if(in_state(super::app_state::AppState::Running)))
            ;
    }
}

fn cursor_as_movement(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Camera>>,
    mut windows: Query<&mut Window>,
) {
    let delta_time = time.delta_secs();
    let mut window = windows.single_mut();

    let width_div_2 = window.resolution.width()/2.;
    let height_div_2 = window.resolution.height()/2.;

    // Games typically only have one window (the primary window)
    if let Some(position) = window.cursor_position() {
        for mut transform in &mut query.iter_mut() {
            transform.rotate_y( delta_time * 50. * (width_div_2  - position.x)/360.);
            transform.rotate_local_x(delta_time * 50. * (height_div_2 - position.y)/360.);
        }
    } else {
        println!("Cursor is not in the game window.");
    }

    window.set_cursor_position(Some((width_div_2, height_div_2).into()));
    // window.set_physical_cursor_position(Some((0.0, 0.0).into()));
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
