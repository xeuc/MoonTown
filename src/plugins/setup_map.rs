
use bevy::asset::AssetLoader;
use bevy::prelude::*;
use bevy::gltf::*;
use bevy::asset::{AssetServer, Assets};
use bevy::render::mesh::VertexAttributeValues;
use bevy_rapier3d::prelude::{Collider, ComputedColliderShape, TriMeshFlags};

pub struct SetupMapPlugin;

impl Plugin for SetupMapPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup)
        .add_systems(Update, find_top_material_and_mesh)
        // .add_systems(Update, move_scene_entities)
        // .add_systems(Startup, spawn_gltf_map)
        // .add_systems(Update, update_colliders)
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

    // let mesh_handles: Vec<Mesh3d> = vec![
    //     Mesh3d(asset_server.load("creative_map.gltf#Mesh1/Primitive0")),
    // ];

    // Cube
    commands.spawn((
        Mesh3d(asset_server.load(GltfAssetLabel::Primitive{mesh: 0, primitive: 0}.from_asset("creative_map.gltf"))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Name::new("cube_mesh"),
        ColliderWaitingForMesh,
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

    // triangle map
    commands.spawn((
        Mesh3d(asset_server.load(GltfAssetLabel::Primitive{mesh: 2, primitive: 0}.from_asset("creative_map.gltf"))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        // Transform(GltfAssetLabel::)
        Name::new("triangle_mesh"),
        ColliderWaitingForMesh,
        // AsyncSceneCollider { ..default() },
    ));

}



fn setup(mut commands: Commands, asset_server: Res<AssetServer>,mut materials: ResMut<Assets<StandardMaterial>>,) {
    
    // Spawn the map
    commands.spawn((
        SceneRoot(
            asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("creative_map.gltf")),
        ),
        ColliderWaitingForMesh,
    ));
}

fn find_top_material_and_mesh(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
    mat_query: Query<(
        Entity,
        &MeshMaterial3d<StandardMaterial>,
        &Mesh3d,
    )>,
) {
    for (ent, mat_handle, mesh_handle) in mat_query.iter() {
        println!("test");
        // locate a material by material name
        // if name.0 == "Top" {
            if let Some(material) = materials.get_mut(mat_handle) {
                if let Color::Hsla(ref mut hsla) = material.base_color {
                    *hsla = hsla.rotate_hue(time.delta_secs() * 100.0);
                } else {
                    material.base_color = Color::from(Hsla::hsl(0.0, 0.9, 0.7));
                }
            }

            if let Some(mesh) = meshes.get_mut(mesh_handle) {
                //
                commands.entity(ent).insert(
                    Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh(TriMeshFlags::from_bits(7u16).unwrap())).unwrap()
                );
                // if let Some(VertexAttributeValues::Float32x3(positions)) =
                //     mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
                // {
                    
                //     for position in positions {
                //         *position = (
                //             position[0],
                //             1.5 + 0.5 * ops::sin(time.elapsed_secs() / 2.0),
                //             position[2],
                //         )
                //             .into();
                //     }
                // }
            }
        // }
        // commands.entity(entity).remove::<ColliderWaitingForMesh>();
    }
}
