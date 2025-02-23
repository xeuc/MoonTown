pub mod player;

pub mod screenshot;
pub mod lost_player;
pub mod fps; 
pub mod char_controller;
pub mod pause;
pub mod cursor_as_movement;
pub mod keyboard;
pub mod setup_map;
pub mod skybox;
pub mod light;
pub mod additional_crate_ui;
pub mod app_state;

pub struct Plugins;

impl Plugin for Plugins {
    fn build(&self, app: &mut App) {
        app
        .add_plugins((
            char_controller::CharacterControllerPlugin,
            lost_player::LostPlayerPlugin,
            screenshot::ScreenshotPlugin,
            cursor_as_movement::RotateHeadPlugin,
            pause::PausePlugin,
            keyboard::KeyboardPlugin,
            skybox::SkyboxPlugin,
            light::LightPlugin,
            fps::FpsPlugin,
            additional_crate_ui::UiPlugin,
            app_state::AppStatePlugin,
            setup_map::SetupMapPlugin,
            player::controls_player_ball::ControlsPlayerBallPlugin,
            player::spawn_player_ball::SpawnPlayerBallPlugin,
        ))
            ;
    }
}