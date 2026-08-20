use bevy::prelude::*;

#[derive(Resource, Default, PartialEq, Eq, Clone, Copy)]
pub enum CameraControlMode {
    #[default]
    WithCursor,
    NoCursor,
}

// switch between moving camera using cursor or not
impl CameraControlMode {
    pub fn toggle(&mut self) {
        *self = match self {
            CameraControlMode::WithCursor => CameraControlMode::NoCursor,
            CameraControlMode::NoCursor => CameraControlMode::WithCursor,
        };
    }
}

// please send the logic in input service + event to toggle
pub fn toggle_control_mode(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mode: ResMut<CameraControlMode>,
) {
    if keyboard.just_pressed(KeyCode::Numpad9) {
        mode.toggle();
    }
}


// Tuto text for user
// Spawn a small hint label in the top-right corner of the screen
pub fn setup_control_hint(mut commands: Commands) {
    commands.spawn((
        Text::new("Press Numpad 9 to switch camera movement"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        // TODO: Check why order 0 Spawn that text in camera 1 🤪
        Camera {
            order: 0,
            ..default()
        },
        TextColor(Color::BLACK),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        },
    ));
}