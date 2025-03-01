
use bevy::prelude::*;
use bevy::asset::{AssetServer, Assets};
use bevy_rapier3d::prelude::{Collider, ComputedColliderShape, TriMeshFlags};

pub struct SetupMapPlugin;

impl Plugin for SetupMapPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup_map)
        .add_systems(Update, find_top_material_and_mesh)
        ;
    }
}


// Not working because it spawn meshes, so the position in the scene is ignored
// To fix it, the "SceneRoot" should be spawn, not the "Mesh3d"
// const MAP_PATH: &str = "creative_map.gltf#Mesh0/Primitive0";
//fn _spawn_gltf_meshes(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // Plane map
//     commands.spawn((
//         Mesh3d(asset_server.load(GltfAssetLabel::Primitive{mesh: 1, primitive: 0}.from_asset("creative_map.gltf"))),
//         MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
//         // Transform broken bc we not using scene
//         Name::new("map_mesh"),
//         EntityWaitingForCollider,
//         // AsyncSceneCollider { ..default() }, // don't work
//     ));
// }



#[derive(Component)]
struct EntityWaitingForCollider;

fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Spawn the map
    commands.spawn((
        SceneRoot(
            asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("creative_map.gltf")),
        ),
        EntityWaitingForCollider,
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
    for (entity, mat_handle, mesh_handle) in mat_query.iter() {
        if let Some(material) = materials.get_mut(mat_handle) {
            if let Color::Hsla(ref mut hsla) = material.base_color {
                *hsla = hsla.rotate_hue(time.delta_secs() * 100.0);
            } else {
                material.base_color = Color::from(Hsla::hsl(0.0, 0.9, 0.7));
            }
        }
        if let Some(mesh) = meshes.get_mut(mesh_handle) {
            // add collider
            commands.entity(entity).insert(
                Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh(TriMeshFlags::from_bits(7u16).unwrap())).unwrap()
            );
        }
        commands.entity(entity).remove::<EntityWaitingForCollider>();
    }
}
