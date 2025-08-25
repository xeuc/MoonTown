
use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;


// SETUP




/// ******************************************************************************
/// ***  MAP  ********************************************************************
/// ******************************************************************************

pub fn _setup_stairs_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    let ground_size = 50.0;
    let ground_height = 0.1;
    commands.spawn((
        Name::new("Ground"),
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
            Name::new("Stair4"),
            Transform::from_xyz(40.0, step * stair_step, step * 2.0 - 20.0),
            cuboid.clone(),
            material.clone(),
            collider.clone(),
        ));
        commands.spawn((
            Name::new("Stair3"),
            Transform::from_xyz(-40.0, step * stair_step, step * -2.0 + 20.0),
            cuboid.clone(),
            material.clone(),
            collider.clone(),
        ));
        commands.spawn((
            Name::new("Stair2"),
            Transform::from_xyz(step * 2.0 - 20.0, step * stair_step, 40.0),
            cuboid.clone(),
            material.clone(),
            collider.clone(),
        ));
        commands.spawn((
            Name::new("Stair1"),
            Transform::from_xyz(step * -2.0 + 20.0, step * stair_step, -40.0),
            cuboid.clone(),
            material.clone(),
            collider.clone(),
        ));
    }

    // Light
    commands.spawn((
        Name::new("Light"),
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



pub fn setup_light(
    mut commands: Commands,
) {

    // Light
    commands.spawn((
        Name::new("Light"),
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









// Setup System
pub fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Spawn the map
    commands.spawn((
        // SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("creative_map_simple3.gltf")),),
        // SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("playtest_map_24x24_scale1.gltf")),),
        // SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("playtest_map_24x24_scale1_test.gltf")),),
        // SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("map_test_tree.gltf")),),
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("playground.gltf")),),
        // ContactSkin(0.2),
        // SoftCcd { prediction: 200. },
        // ColliderScale(1.2),
        Transform::from_xyz(0.2,2., 0.2).with_scale(Vec3::splat(10.)),

        AsyncSceneCollider::default(),
        RigidBody::Fixed,
    ));


    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("tree.gltf")),),
        Transform::from_xyz(-50.,0., 50.).with_scale(Vec3::splat(1.)),
        AsyncSceneCollider::default(),
        RigidBody::Fixed,
    ));
    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("tree.gltf")),),
        Transform::from_xyz(-50.,0.,-50.).with_scale(Vec3::splat(1.)),
        AsyncSceneCollider::default(),
        RigidBody::Fixed,
    ));
    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("tree.gltf")),),
        Transform::from_xyz(50.,0., 50.).with_scale(Vec3::splat(1.)),
        AsyncSceneCollider::default(),
        RigidBody::Fixed,
    ));
    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("tree.gltf")),),
        Transform::from_xyz(50.,0., -50.).with_scale(Vec3::splat(1.)),
        AsyncSceneCollider::default(),
        RigidBody::Fixed,
    ));


}