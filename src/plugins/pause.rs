use bevy::prelude::*;
// use std::time::Duration;

use super::app_state::AppState;



pub struct PausePlugin;
impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, pause_button_system)
            .add_systems(OnEnter(AppState::Paused), on_paused)
            .add_systems(OnExit(AppState::Paused), on_unpaused)
            .add_systems(Update, while_paused.run_if(in_state(AppState::Paused)))
            ;
    }
}

fn while_paused(mut _time: ResMut<Time>) {
}

fn on_paused(mut _time: ResMut<Time>) {
}

fn on_unpaused(mut _time: ResMut<Time>) {
    // time.delta();
}

fn pause_button_system(
    current_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyQ) {
        let new_state = match *current_state.get() {
            // AppState::Loading => AppState::Loading,
            AppState::Paused => AppState::Running,
            AppState::Running => AppState::Paused,
        };
        next_state.set(new_state);
    }
}

