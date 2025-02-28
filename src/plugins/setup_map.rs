
use bevy::prelude::*;
use bevy::asset::{AssetServer, Assets};
use bevy_rapier3d::prelude::{Collider, ComputedColliderShape, TriMeshFlags};

pub struct SetupMapPlugin;

impl Plugin for SetupMapPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_gltf_map)
        .add_systems(Update, update_colliders)
        ;
    }
}




#[derive(Component)]
struct ColliderWaitingForMesh;

// const MAP_PATH: &str = "nulMap4.gltf#Mesh0/Primitive0";
// const MAP_PATH: &str = "creative_map.gltf#Mesh0/Primitive0";
const MAP_PATH: &str = "creative_map.gltf#Mesh0/Primitive0";


fn spawn_gltf_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // let mesh_handle: Mesh3d = Mesh3d(asset_server.load(MAP_PATH));
    let mesh_handles: Vec<Mesh3d> = vec![
        Mesh3d(asset_server.load("creative_map.gltf#Mesh1/Primitive0")),
        // The commented one is a LARGE plan, so player clip really easily through it. 
        // Mesh3d(asset_server.load("creative_map2.gltf#Mesh1/Primitive0")),
        // Mesh3d(asset_server.load("creative_map3.gltf#Mesh1/Primitive0")),

        Mesh3d(asset_server.load("creative_map.gltf#Mesh0/Primitive0")),
        // Mesh3d(asset_server.load("creative_map.gltf#Mesh2/Primitive0")),
        // Mesh3d(asset_server.load("creative_map.gltf#Mesh3/Primitive0")),
    ];
    // let mesh_handle =
    //     asset_server.load(GltfAssetLabel::Primitive{mesh: 0, primitive: 0}.from_asset("creative_map.gltf"))
    //     //asset_server.load(GltfAssetLabel::Mesh(0).from_asset("../../../assets/cube.gltf"))
    //     //meshes.add(Cuboid::new(1.0, 1.0, 1.0)).into()
    // ;

    // for mesh_handle in &mesh_handles {
    //     commands.spawn((
    //         mesh_handle.clone(),
    //         ColliderWaitingForMesh,
    //     ));
    // }
    // commands.spawn((
    //     SceneRoot(asset_server.load(
    //         GltfAssetLabel::Scene(0).from_asset(MAP_PATH),
    //     )),
    //     Name::new("map_scene"),
    //     // mesh_handle.clone(), // ???
    //     // ColliderWaitingForMesh,
    // ));

    // Cube
    commands.spawn((
        Mesh3d(asset_server.load(GltfAssetLabel::Primitive{mesh: 0, primitive: 0}.from_asset("creative_map.gltf"))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Name::new("cube_mesh"),
        ColliderWaitingForMesh,
        // AsyncSceneCollider { ..default() },
    ));

    // Plane map
    commands.spawn((
        Mesh3d(asset_server.load(GltfAssetLabel::Primitive{mesh: 1, primitive: 0}.from_asset("creative_map.gltf"))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        // Transform(GltfAssetLabel::)
        Name::new("map_mesh"),
        ColliderWaitingForMesh,
        // AsyncSceneCollider { ..default() },
    ));

    // commands.spawn((
    //     Mesh3d(asset_server.load(GltfAssetLabel::Primitive{mesh: 2, primitive: 0}.from_asset("creative_map.gltf"))),
    //     MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    //     Name::new("map_mesh"),
    //     ColliderWaitingForMesh,
    // ));
}

// fn update_colliders(
//     mut commands: Commands,
//     meshes: Res<Assets<Mesh>>,
//     query: Query<(Entity, &Mesh3d), With<ColliderWaitingForMesh>>,
// ) {
//     for (entity, mesh_handle) in query.iter() {
//         if let Some(mesh) = meshes.get(mesh_handle) {
//             // This is transforms applied by blender to nulmap4, so not needed for others maps !
//             // TODO: found a way to do it automatically
//             // let map_transform = Transform {
//             //     translation: Vec3::new(-215.70999145507812, 0.0, 0.0),
//             //     scale: Vec3::new(1024.0, 1.0, 1024.0),
//             //     ..Default::default()
//             // };
//             // let map_mesh = mesh.clone().transformed_by(map_transform);

            
//             let map_mesh = mesh.clone();
//             commands.entity(entity).insert(
//                 Collider::from_bevy_mesh(&map_mesh, &ComputedColliderShape::TriMesh(TriMeshFlags::from_bits(7u16).unwrap())).unwrap()
//             );

//             commands.entity(entity).remove::<ColliderWaitingForMesh>();
//         }
//     }
// }

fn update_colliders(
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
    query: Query<(Entity, &Mesh3d, &GlobalTransform), With<ColliderWaitingForMesh>>, // Ajout du Transform
) {
    for (entity, mesh_handle, global_transform) in query.iter() {
        if let Some(mesh) = meshes.get(mesh_handle) {
            let map_mesh = mesh.clone();

            println!("{:?}", global_transform);

            // let map_transform = Transform {
            //     translation: Vec3::new(0.0625,-0.25,0.65625),
            //     // scale: Vec3::new(1024.0, 1.0, 1024.0),
            //     ..Default::default()
            // };
            // Appliquer le Transform au mesh
            // let transformed_mesh = map_mesh.transformed_by(map_transform);
            let transformed_mesh = map_mesh.transformed_by((*global_transform).into());

            // Générer le collider avec la transformation
            commands.entity(entity).insert(
                Collider::from_bevy_mesh(&transformed_mesh, &ComputedColliderShape::TriMesh(TriMeshFlags::from_bits(7u16).unwrap())).unwrap()
            );

            commands.entity(entity).remove::<ColliderWaitingForMesh>();
        }
    }
}






