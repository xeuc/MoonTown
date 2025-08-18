use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy::{prelude::*, pbr::CascadeShadowConfigBuilder};
use bevy_rapier3d::{control::KinematicCharacterController, prelude::*};
use std::f32::consts::PI;

use crate::utils::{CameraPosition, Direction, Player, RotateCamera, TopLeftCamera};
use crate::utils::PlayerCamera;




/// ******************************************************************************
/// ***  MISC  *******************************************************************
/// ******************************************************************************

// make cursor don't move and invisible
pub fn cursor_grab( 
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let Ok(mut primary_window) = q_windows.single_mut() else { return;};
    // primary_window.cursor_options.grab_mode = CursorGrabMode::Confined;
    primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor_options.visible = false;
}












/// ******************************************************************************
/// ***  PLAYER + CAM0 ***********************************************************
/// ******************************************************************************

// Setup player entity and its integrated camera
pub fn setup_player_camera_integrated(mut commands: Commands) {
    
    // Player
    commands.spawn((
        Transform::from_xyz(0.0, 5.0, 0.0),
        Visibility::default(),
        Player,
        Collider::round_cylinder(0.9, 0.3, 0.2),
        KinematicCharacterController {
            custom_mass: Some(5.0),
            up: Vec3::Y,
            offset: CharacterLength::Absolute(0.01),
            slide: true,
            autostep: Some(CharacterAutostep {
                max_height: CharacterLength::Relative(0.3),
                min_width: CharacterLength::Relative(0.5),
                include_dynamic_bodies: false,
            }),
            // Don’t allow climbing slopes larger than 45 degrees.
            max_slope_climb_angle: 45.0_f32.to_radians(),
            // Automatically slide down on slopes smaller than 30 degrees.
            min_slope_slide_angle: 30.0_f32.to_radians(),
            apply_impulse_to_dynamic_bodies: true,
            snap_to_ground: None,
            ..default()
        },
        
    ));

    // Player Camera
    commands.spawn((
        PlayerCamera,
        Camera3d::default(), 
        Transform::from_xyz(0.0, 2.0, 5.0).looking_at((0.0, 5.0, 0.0).into(), Vec3::Y),
        Camera {
            // Renders cameras with different priorities to prevent ambiguities
            order: 0,
            ..default()
        },
    ));


    
}












/// ******************************************************************************
/// ***  MAP  ********************************************************************
/// ******************************************************************************

pub fn setup_maps_elements(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    let ground_size = 50.0;
    let ground_height = 0.1;
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0*ground_size, 2.0*ground_height, 2.0*ground_size))),
        // MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, -ground_height, 0.0),
        Collider::cuboid(ground_size, ground_height, ground_size),
    ));



    // Stairs-Wall
    let stair_len = 30;
    let stair_step = 0.2;
    for i in 1..=stair_len {
        let step = i as f32;
        let collider = Collider::cuboid(1.0, step * stair_step, 1.0);
        let cuboid = Mesh3d(meshes.add(Cuboid::new(2.0*1.0, 2.0*step * stair_step, 2.0*1.0)));
        let material= MeshMaterial3d(materials.add(Color::srgb(153.0/255.0, 90.0/255.0, 50.0/255.0)));
        commands.spawn((
            Transform::from_xyz(40.0, step * stair_step, step * 2.0 - 20.0),
            cuboid.clone(),
            material.clone(),
            collider.clone(),
        ));
        commands.spawn((
            Transform::from_xyz(-40.0, step * stair_step, step * -2.0 + 20.0),
            cuboid.clone(),
            material.clone(),
            collider.clone(),
        ));
        commands.spawn((
            Transform::from_xyz(step * 2.0 - 20.0, step * stair_step, 40.0),
            cuboid.clone(),
            material.clone(),
            collider.clone(),
        ));
        commands.spawn((
            Transform::from_xyz(step * -2.0 + 20.0, step * stair_step, -40.0),
            cuboid.clone(),
            material.clone(),
            collider.clone(),
        ));
    }

    // Light
    commands.spawn((
        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        CascadeShadowConfigBuilder {
            num_cascades: 2,
            first_cascade_far_bound: 200.0,
            maximum_distance: 280.0,
            ..default()
        }
        .build(),
    ));
}









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
            Transform::from_translation(Vec3::new(0.0, 40.0, -60.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            Camera {
                // Renders cameras with different priorities to prevent ambiguities
                order: 1 as isize,
                ..default()
            },
            TopLeftCamera,
            CameraPosition {
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
            // if yo udon't put the node element,
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


