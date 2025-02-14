use bevy::{
    prelude::*,
    render::view::screenshot::{save_to_disk, Capturing, Screenshot},
    window::SystemCursorIcon,
    winit::cursor::CursorIcon,
};

use chrono::{Datelike, Timelike, Utc};

pub struct ScreenshotPlugin;


impl Plugin for ScreenshotPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, create_screenshot_folder)
            .add_systems(Update, screenshot_button)
            .add_systems(Update, screenshot_saving);
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
    // main_window: Query<Entity, With<PrimaryWindow>>,
    // mut screenshot_manager: ResMut<ScreenshotManager>,
    mut counter: Local<u32>,

    mut commands: Commands,
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

        // screenshot_manager // Old way
        //     .save_screenshot_to_disk(main_window.single(), path)
        //     .unwrap();

        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
    }
}



fn screenshot_saving(
    mut commands: Commands,
    screenshot_saving: Query<Entity, With<Capturing>>,
    windows: Query<Entity, With<Window>>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    match screenshot_saving.iter().count() {
        0 => {
            commands.entity(window).remove::<CursorIcon>();
        }
        x if x > 0 => {
            commands
                .entity(window)
                .insert(CursorIcon::from(SystemCursorIcon::Progress));
        }
        _ => {}
    }
}




