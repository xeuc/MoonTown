use bevy::{asset::RenderAssetUsages, core::Name, core_pipeline::Skybox, prelude::*, render::render_resource::{Extent3d, TextureDimension, TextureFormat}};
use bevy_rapier3d::prelude::*;

pub struct SpawnPlayerBallPlugin;

impl Plugin for SpawnPlayerBallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            // Enable this will show you gizmos of the colider, but will slow down the game
            // (slow down more with high number of polygons)
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(Startup, setup)
            ;
    }
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}



fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let skybox_handle = assets.load(super::super::skybox::CUBEMAPS[0].0); // TODO
    
    
    // Collider::cylinder(0.5, 0.25),


    // spawn capsule player (wasd)
    commands
        .spawn((
            // Mesh3d(meshes.add(Sphere::default().mesh().uv(16, 10))),
            Visibility::default(),
            MeshMaterial3d(debug_material.clone()),
            RigidBody::KinematicPositionBased,
            // Collider::capsule_y(1., 0.5),
            Collider::ball(1.),
            Transform::from_xyz(0.0, 3.0, 0.0).with_scale(Vec3::splat(0.001)),
            super::super::super::Player, // TODO fix the super super super...
            Name::new("Player"),
            // ContactSkin(0.2),
            SoftCcd { prediction: 5.0 },
            KinematicCharacterController {
                offset: CharacterLength::Relative(0.01),
                snap_to_ground: Some(CharacterLength::Absolute(0.5)),
                ..KinematicCharacterController::default()
            },
        ))
        .with_child((
            // Transform::from_xyz(1., 1., 5.).looking_at(Vec3::from_array([1., -55., 5.]), Vec3::Y),
            Transform::from_xyz(1., 1., 5.),
            Camera3d {
                ..default()
            },
            Skybox {
                image: skybox_handle.clone(),
                brightness: 1000.0,
                rotation:  Quat::IDENTITY,
            },
        ))
        ;
}
