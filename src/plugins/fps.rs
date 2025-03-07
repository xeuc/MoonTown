//! Showcase how to use and configure FPS overlay.

use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    text::FontSmoothing,
};

struct OverlayColor;

impl OverlayColor {
    const RED: Color = Color::srgb(1.0, 0.0, 0.0);
    const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
}

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                FpsOverlayPlugin {
                    config: FpsOverlayConfig {
                        text_config: TextFont {
                            // Here we define size of our overlay
                            font_size: 42.0,
                            // If we want, we can use a custom font
                            font: default(),
                            // We could also disable font smoothing,
                            font_smoothing: FontSmoothing::default(),
                        },
                        // We can also change color of the overlay
                        text_color: OverlayColor::GREEN,
                        enabled: true,
                    },
                },
            ))
            .add_systems(Update, customize_config);
        
    }
}


fn customize_config(input: Res<ButtonInput<KeyCode>>, mut overlay: ResMut<FpsOverlayConfig>) {
    if input.just_pressed(KeyCode::Digit1) {
        // Changing resource will affect overlay
        if overlay.text_color == OverlayColor::GREEN {
            overlay.text_color = OverlayColor::RED;
        } else {
            overlay.text_color = OverlayColor::GREEN;
        }
    }
    if input.just_pressed(KeyCode::Digit2) {
        overlay.text_config.font_size -= 2.0;
    }
    if input.just_pressed(KeyCode::Digit3) {
        overlay.text_config.font_size += 2.0;
    }
    if input.just_pressed(KeyCode::Digit4) {
        overlay.enabled = !overlay.enabled;
    }
}
