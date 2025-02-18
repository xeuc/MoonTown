
use bevy::prelude::*;
use bevy::render::mesh::{Indices, VertexAttributeValues};

use bevy::asset::{AssetServer, Assets};
use bevy::core_pipeline::Skybox;


use bevy_rapier3d::prelude::Collider;

#[derive(Default)]
struct Player {
    entity: Option<Entity>,
}
#[derive(Resource, Default)]
struct Game {
    player: Player,
}



pub struct SetupMapPlugin;

impl Plugin for SetupMapPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<Game>()
        .add_systems(Startup, load_gltf_meshes)
        .add_systems(Startup, cubemap_setup)
        
        .add_systems(Update, process_gltf_meshes)
        ;
    }
}

fn cubemap_setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut game: ResMut<Game>,
) {
    let skybox_handle = assets.load(super::skybox::CUBEMAPS[0].0); // TODO
    let entity_player = commands.spawn((
        Camera3d {
            // transform: Transform::from_xyz(0.0, 100., 0.0),
            ..default()
        },
        Skybox {
            image: skybox_handle.clone(),
            brightness: 1000.0,
            rotation:  Quat::IDENTITY,
            },
    ))
    .id()
    ;

    game.player.entity = Some(entity_player);
}


#[derive(Resource)]
struct GltfMeshHandle {
    handle: Handle<Mesh>,
}



fn load_gltf_meshes(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mesh_handle: Handle<Mesh> = asset_server.load("nulMap4.gltf#Mesh0/Primitive0");
    commands.insert_resource(GltfMeshHandle { handle: mesh_handle });
}

fn process_gltf_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    gltf_mesh_handle: Option<Res<GltfMeshHandle>>,
) {
    println!("-1");
    if let Some(gltf_mesh_handle) = &gltf_mesh_handle {
        let mut new_meshes = Vec::new();
        println!("0");
        if let Some(mesh) = meshes.get(&gltf_mesh_handle.handle) {
            // mesh.duplicate_vertices();
            println!("1");
            if let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
                println!("2");
                // Stop here for some reason
                // if let Some(Indices::U32(indices)) = mesh.indices() {
                    println!("3");
                    for triangle in positions {
                        // println!("4");
                        if let [i0, i1, i2] = triangle {
                            // println!("5");
                            let v0 = positions[*i0 as usize];
                            let v1 = positions[*i1 as usize];
                            let v2 = positions[*i2 as usize];
                            // println!("{:?}", v0);

                            let triangle_mesh = Mesh::from(Triangle3d {
                                vertices: [
                                    Vec3::from(v0),
                                    Vec3::from(v1),
                                    Vec3::from(v2),
                                ],
                            });

                            new_meshes.push((triangle_mesh, Vec3::from(v0), Vec3::from(v1), Vec3::from(v2)));
                            commands.remove_resource::<GltfMeshHandle>();

                        }
                    }
                // }
            }
        }

        // println!("{:?}", new_meshes);
        println!(":)");

        for ( triangle_mesh, v0,v1,v2) in new_meshes {
            let triangle_mesh_handle = meshes.add(triangle_mesh);
            commands.spawn((
                Mesh3d(triangle_mesh_handle.clone()),
                Collider::triangle(v0,v1,v2),
            ));
        }

        

    }
}




// fn update_colliders(
//     mut commands: Commands,
//     meshes: Res<Assets<Mesh>>,
//     query: Query<(Entity, &Mesh3d), With<ColliderWaitingForMesh>>,
// ) {
//     for (entity, mesh_handle) in query.iter() {
//         if let Some(mesh) = meshes.get(mesh_handle) {

//             let map_transform = Transform {
//                 translation: Vec3::new(-215.70999145507812, 0.0, 0.0), // Déplacement
//                 scale: Vec3::new(1024.0, 1.0, 1024.0), // Échelle correcte
//                 ..Default::default() // Garder la rotation par défaut
//             };

//             commands.entity(entity).remove::<ColliderWaitingForMesh>();
//         }
//     }
// }






















