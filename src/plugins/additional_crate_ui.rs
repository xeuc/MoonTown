use bevy::prelude::*;
use bevy_egui::egui::Pos2;
// use bevy_egui::*;

use bevy_egui::{egui, EguiContexts, EguiPlugin};


pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(EguiPlugin)
        .init_resource::<SliderValueSmiley>()
            .add_systems(Update, ui_example_system);
    }
}

#[derive(Resource, Default)]
pub struct SliderValueSmiley{
    slider_value_smiley: f32,
}

fn ui_example_system(
    mut query: Query<&mut Transform, With<Camera>>,
    mut slider_value_smiley: ResMut<SliderValueSmiley>,
    mut contexts: EguiContexts
) {
    egui::Window::new("Hello")
        .fixed_pos(Pos2::new(0., 40.))
        .show(contexts.ctx_mut(), |ui| {
            ui.label("Rotationnate the shape:");
            ui.style_mut().spacing.slider_width = 300.0;

            ui.add(egui::Slider::new(&mut slider_value_smiley.slider_value_smiley, 0.0..=std::f32::consts::PI*2.0).text("My value"));
            
            for transform in &mut query {
                let position_text = format!(
                    "Camera position is x={}, y={}, z={}",
                    transform.translation.x,
                    transform.translation.y,
                    transform.translation.z
                );
                ui.label(position_text.clone());

                let position_text = format!(
                    "Camera rotation is x={}, y={}, z={}",
                    transform.rotation.x,
                    transform.rotation.y,
                    transform.rotation.z
                );
                ui.label(position_text.clone());
            }
        }
    );
}