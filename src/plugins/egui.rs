use bevy::prelude::*;
use bevy_egui::egui::Pos2;
// use bevy_egui::*;

use bevy_egui::{egui, EguiContexts, EguiPlugin};
// use bevy_rapier3d::prelude::KinematicCharacterControllerOutput;


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
    // mut query2: Query<&mut Transform, With<super::super::Player>>,
    mut slider_value_smiley: ResMut<SliderValueSmiley>,
    mut contexts: EguiContexts,
    // controllers: Query<(Entity, &KinematicCharacterControllerOutput, &Transform), Without<Camera>>,
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
            // for transform in &mut query2 {
            //     let position_text = format!(
            //         "Player position is x={}, y={}, z={}",
            //         transform.translation.x,
            //         transform.translation.y,
            //         transform.translation.z
            //     );
            //     ui.label(position_text.clone());

            //     let position_text = format!(
            //         "Player rotation is x={}, y={}, z={}",
            //         transform.rotation.x,
            //         transform.rotation.y,
            //         transform.rotation.z
            //     );
            //     ui.label(position_text.clone());
            // }
            
            // for (entity, output, transform2) in controllers.iter() {
            //     let ball_info_text = format!(
            //         "Entity: {:?}, Moved by {:?}, Grounded: {:?}, Position: {:?}",
            //         entity, output.effective_translation, output.grounded, transform2.translation
            //     );
            //     ui.label(ball_info_text);
            // }

        }
    );
}


