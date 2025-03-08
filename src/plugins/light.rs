use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};
use std::f32::consts::PI;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 2000.,
            })
            .add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
) {
    // Light
    commands.spawn((
        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 200.0,
            maximum_distance: 400.0,
            ..default()
        }
        .build(),
    ));
}