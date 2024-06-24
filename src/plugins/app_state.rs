use bevy::prelude::*;


#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    // #[default]
    // Loading,
    Paused,
    #[default]
    Running,
}

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>();
    }
}