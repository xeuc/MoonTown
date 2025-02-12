use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*};
pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_fps_2)
            .add_systems(Update, update_log_text);
    }
}

#[derive(Component)]
struct FpsText;

fn setup_fps_2(
    mut commands: Commands,
) {
    commands.spawn(
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        },
    )
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                format!(""),

                TextStyle {
                    font_size: 20.0,
                    color: Color::rgb(0.5, 0.5, 1.0),
                    ..default()
                },
            ).with_background_color(Color::rgb(1., 1., 1.)),
            FpsText,));
        },
    
    );
}


fn _setup_fps(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "FPS: ",
            TextStyle {
                font: default(),
                font_size: 20.0,
                color: Color::TOMATO,
                
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
        FpsText,
    ));
}


fn update_log_text(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
    mut cam_transforms: Query<&mut Transform, With<Camera>>, // TODO: do not use querry here
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[0].value = format!("FPS: {value:.2}\n");
            }
        }

        for cam_transform in &mut cam_transforms.iter_mut() {
            // append cam pos
            text.sections[0].value += &format!("CAM_POS_X: {}\n", cam_transform.translation.x);//camTransform.translation.x
            text.sections[0].value += &format!("CAM_POS_Y: {}\n", cam_transform.translation.y);//camTransform.translation.x
            text.sections[0].value += &format!("CAM_POS_Z: {}\n", cam_transform.translation.z);//camTransform.translation.x
            text.sections[0].value += &format!("COUCOU :D");//camTransform.translation.x
        }

    }
}