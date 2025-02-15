use bevy::prelude::*;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(
        DirectionalLight {
            ..default()
        },
        // transform: Transform::from_xyz(0.0, 2.0, 0.0)
        //     .with_rotation(Quat::from_rotation_x(-PI / 4.)),
        // ..default()
        // }
    );
}