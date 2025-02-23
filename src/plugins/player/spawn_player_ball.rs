use bevy::{asset::RenderAssetUsages, prelude::*, render::render_resource::{Extent3d, TextureDimension, TextureFormat}};
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    // spawn ball player (numpad)
    commands
        .spawn(Mesh3d(meshes.add(Sphere::default().mesh().uv(32, 18))))
        .insert(MeshMaterial3d(debug_material.clone()),)
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(0.5))
        .insert(Transform::from_xyz(0.0, 10.0, 0.0))
        .insert(KinematicCharacterController {
            ..KinematicCharacterController::default()
        });
}
