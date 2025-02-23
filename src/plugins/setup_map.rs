
use bevy::prelude::*;
use bevy::asset::{AssetServer, Assets};
use bevy::core_pipeline::Skybox;
use bevy_rapier3d::prelude::{Collider, ComputedColliderShape, TriMeshFlags};

pub struct SetupMapPlugin;

impl Plugin for SetupMapPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, cubemap_setup)
        .add_systems(Startup, spawn_gltf_map)
        .add_systems(Update, update_colliders)
        // .add_systems(Update, check_mesh_ready_no_rapier)
        ;
    }
}

fn cubemap_setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let skybox_handle = assets.load(super::skybox::CUBEMAPS[0].0); // TODO
    commands.spawn((
        Camera3d {
            // transform: Transform::from_xyz(0.0, 100., 0.0),
            ..default()
        },
        Skybox {
            image: skybox_handle.clone(),
            brightness: 1000.0,
            rotation:  Quat::IDENTITY,
            },
    ));
}



#[derive(Component)]
struct ColliderWaitingForMesh;

const MAP_PATH: &str = "nulMap4.gltf#Mesh0/Primitive0";


fn spawn_gltf_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    let mesh_handle: Mesh3d = bevy::prelude::Mesh3d(asset_server.load(MAP_PATH));

    commands.spawn((
        SceneRoot(asset_server.load(
            GltfAssetLabel::Scene(0).from_asset(MAP_PATH),
        )),
        mesh_handle.clone(), // ???
        ColliderWaitingForMesh,
    ));
}

fn update_colliders(
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
    query: Query<(Entity, &Mesh3d), With<ColliderWaitingForMesh>>,
) {
    for (entity, mesh_handle) in query.iter() {
        if let Some(mesh) = meshes.get(mesh_handle) {
            let map_transform = Transform {
                translation: Vec3::new(-215.70999145507812, 0.0, 0.0),
                scale: Vec3::new(1024.0, 1.0, 1024.0),
                ..Default::default()
            };
            let map_mesh = mesh.clone().transformed_by(map_transform);
            commands.entity(entity).insert(
                Collider::from_bevy_mesh(&map_mesh, &ComputedColliderShape::TriMesh(TriMeshFlags::from_bits(1u16).unwrap())).unwrap()
            );

            commands.entity(entity).remove::<ColliderWaitingForMesh>();
        }
    }
}






