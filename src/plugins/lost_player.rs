
use bevy::{app::Plugin, ecs::{query::With, system::Query}, transform::components::Transform};
use bevy::prelude::*;

use crate::Player;
pub struct LostPlayerPlugin;

impl Plugin for LostPlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, tp_when_player_below_y);
    }
}

fn tp_when_player_below_y(
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut transform in &mut query.iter_mut() {
        if transform.translation.y > -200. {
            return
        }
        transform.translation = Transform::from_xyz(0.0, 10., 0.0).translation;
    }
}