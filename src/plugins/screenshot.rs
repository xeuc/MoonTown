use bevy::{prelude::*, render::view::screenshot::ScreenshotManager, window::PrimaryWindow};
use chrono::{Datelike, Timelike, Utc};

pub struct ScreenshotPlugin;


impl Plugin for ScreenshotPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, create_screenshot_folder)
            .add_systems(Update, screenshot_button);
    }
}

use std::fs;
use std::path::Path;

// TODO To put in config file
fn create_screenshot_folder(/*path: &str*/) {
    // let path = Path::new(path);
    let path = Path::new("screenshots");
    if !path.exists() {
        fs::create_dir_all(path).expect("Failed to create directory");
    }
}

fn screenshot_button(
    input: Res<ButtonInput<KeyCode>>,
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    mut counter: Local<u32>,
) {
    if input.just_pressed(KeyCode::NumpadDecimal) {
        let now = Utc::now();
        let path = format!(
            "screenshots/ScreenshotThe{}-{:02}-{:02}-{:?}At{}.{}.{}Num{}.png",
            now.year_ce().1, now.month(),
            now.day(),       now.weekday(),
            now.hour12().1,  now.minute(),
            now.second(),    *counter
        );
        *counter += 1;
        screenshot_manager
            .save_screenshot_to_disk(main_window.single(), path)
            .unwrap();
    }
}





