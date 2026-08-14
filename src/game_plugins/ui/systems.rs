use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy::window::WindowResized;


use crate::game_plugins::ui::components::*;


// SPAWNER


/// ******************************************************************************
/// ***  UI = CAM2 in top right corner  ******************************************
/// ******************************************************************************


// set up the UI GFY
pub fn setup_ui(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Cameras and their dedicated UI
    let camera = commands
        .spawn((
            Camera3d::default(),
            Transform::from_translation(Vec3::new(0.0, 3.0, -30.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            Camera {
                // Renders cameras with different priorities to prevent ambiguities
                order: 1 as isize,
                ..default()
            },
            TopLeftCamera,
            CameraViewUIPositionOnScreen {
                pos: UVec2::new((0 % 2) as u32, (0 / 2) as u32),
            },
        ))
        .id();

    // Set up UI (Little rectangle at the top left corner)
    // Wrapper of both button PLUS TEXT that have the same portion as it's parent
    // dynamicly set IG 
    // It's invisible and just help to set the position of the 2 buttons
    commands
        .spawn((
            UiTargetCamera(camera),
            // if you don't put the node element,
            // WARN bevy_ecs::hierarchy: warning[B0004]: 
            // Entity 134v1 with the GlobalTransform component has a parent without GlobalTransform
            // WARN bevy_ecs::hierarchy: warning[B0004]: 
            // Entity 134v1 with the InheritedVisibility component has a parent without InheritedVisibility.
            Node {
                // 100% is 100% of the rectangle
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Player 1"),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(12.),
                    left: Val::Px(12.),
                    // height and width are dynamicly set by
                    // set_camera_viewports function
                    ..default()
                },
            ));
            buttons_panel_spawner(parent);
        });

    // Wrapper of both button that have the same portion as it's parent
    // It's invisible and just help to set the position of the 2 buttons
    fn buttons_panel_spawner(parent: &mut ChildSpawnerCommands) {
        parent
            .spawn(Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.)),
                ..default()
            })
            .with_children(|parent| {
                rotate_button_spawner(parent, "<", Direction::Left);
                rotate_button_spawner(parent, ">", Direction::Right);
            });
    }

    fn rotate_button_spawner(parent: &mut ChildSpawnerCommands, caption: &str, direction: Direction) {
        parent
            .spawn((
                RotateCamera(direction),
                Button,
                Node {
                    width: Val::Px(40.),
                    height: Val::Px(40.),
                    border: UiRect::all(Val::Px(2.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(Color::WHITE),
                BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
            ))
            .with_children(|parent| {
                parent.spawn(Text::new(caption));
            });
    }
}


















// UPDATES



// Osef
// Rotate button behavior
pub fn rotate_button_behavior(
    interaction_query: Query<
        (&Interaction, &ComputedNodeTarget, &RotateCamera),
        (Changed<Interaction>, With<Button>),
    >,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    for (interaction, computed_target, RotateCamera(direction)) in &interaction_query {
        if let Interaction::Pressed = *interaction {
            // Since TargetCamera propagates to the children, we can use it to find
            // which side of the screen the button is on.
            if let Some(mut camera_transform) = computed_target
                .camera()
                .and_then(|camera| camera_query.get_mut(camera).ok())
            {
                let angle = match direction {
                    Direction::Left => -0.1,
                    Direction::Right => 0.1,
                };
                camera_transform.rotate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::Y, angle));
            }
        }
    }
}



// Osef
// If windows resize => Then update little rectangle
pub fn resize_little_rectangle(
    windows: Query<&Window>,
    mut resize_events: EventReader<WindowResized>,
    mut query: Query<(&CameraViewUIPositionOnScreen, &mut Camera), With<TopLeftCamera>>,
) {
    // We need to dynamically resize the camera's viewports whenever the window size changes
    // so then each camera always takes up half the screen.
    // A resize_event is sent when the window is first created, allowing us to reuse this system for initial setup.
    for resize_event in resize_events.read() {
        let window = windows.get(resize_event.window).unwrap();
        let size = window.physical_size()/4 ;
        // let size = UVec2::new(size.x/2 as u32, size.y as u32);

        for (camera_position, mut camera) in &mut query {
            camera.viewport = Some(Viewport {
                physical_position: camera_position.pos * size,
                physical_size: size,
                ..default()
            });
        }
    }
}
