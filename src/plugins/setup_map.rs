use std::borrow::BorrowMut;
use std::default;

use bevy::prelude::*;
use bevy::render::mesh::{Indices, VertexAttributeValues};

use bevy::asset::{AssetServer, Assets, Handle, LoadState};
use bevy::core_pipeline::Skybox;

use rand::*;



// #[derive(Debug, Clone, Eq, PartialEq, Hash)]
// pub enum AppState {
//     #[default]
//     Loading,
//     Game,
// }

pub struct SetupMapPlugin;

impl Plugin for SetupMapPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup2)
        .add_systems(Startup, setup)

        // .add_systems(Startup, check_asset_load_state.after(setup2))
        
        // .add_systems(Update, make_bevy_wait_bc_he_does_not_know_how)
        // .add_systems(Update, check_mesh_ready_no_rapier.after(make_bevy_wait_bc_he_does_not_know_how))

        .add_systems(Update, check_mesh_ready_no_rapier)
        // .add_systems(Update, bon_ta_gagne_voila_ton_update_de_merde)
        ;
    }
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    // Player
    let skybox_handle = assets.load(super::skybox::CUBEMAPS[0].0); // TODO
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 100., 0.0),
            ..default()
        },
        Skybox {
            image: skybox_handle.clone(),
            brightness: 1000.0,
        },
    ))
    ;
}






#[derive(Resource)]
struct MeshHandle {
    handle: Handle<Mesh>,
}

fn setup2(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Charge le handle du mesh
    // let mesh_handle = asset_server.load("triangle_simple.gltf#Mesh0/Primitive0");
    // let mesh_handle = asset_server.load("triangle_simple_2.gltf#Mesh1/Primitive0");
   
    let mesh_handle = asset_server.load("nulMap4.gltf#Mesh0/Primitive0");

    commands.insert_resource(MeshHandle { handle: mesh_handle });

    // Charge la scène GLTF
    commands.spawn(SceneBundle {
        // scene: asset_server.load("triangle_simple.gltf#Scene0"),
        // scene: asset_server.load("triangle_simple_2.gltf#Scene0"),
        scene: asset_server.load("nulMap4.gltf#Scene0"),
        // transform: Transform::from_xyz(0.0, 1000., 0.0),
        ..default()
    });
}





// fn make_bevy_wait_bc_he_does_not_know_how() { }

// Définir un composant pour marquer les entités déjà randomisées
#[derive(Component)]
struct Randomized;

fn check_mesh_ready_no_rapier(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Handle<Mesh>, &Handle<StandardMaterial>), Without<Randomized>>,
) {
    // println!("START FUNCTION check_mesh_ready_no_rapier");
    for (entity, mesh_handle, material_handle) in query.iter() {
        if let Some(standard_material) = materials.get_mut(material_handle) {
            standard_material.base_color = Color::rgb(1.0, 1.0, 1.0); // WHITE
        }

        if let Some(mut mesh) = meshes.get_mut(mesh_handle) {
                        // // Debug output before duplicating vertices
                        // if mesh.indices().is_some() {
                        //     println!("Indices found before duplication.");
                        // } else {
                        //     println!("No indices found before duplication.");
                        // }
            
                        // // Duplicate vertices to ensure no vertices are shared
                        // mesh.duplicate_vertices();
            
                        // // Debug output after duplicating vertices
                        // if mesh.indices().is_some() {
                        //     println!("Indices found after duplication.");
                        // } else {
                        //     println!("No indices found after duplication.");
                        // }
            if let Some(VertexAttributeValues::Float32x3(positions)) = mesh.clone().attribute(Mesh::ATTRIBUTE_POSITION) {
                println!("Number of vertices: {}", positions.len());

                if let Some(indices) = mesh.indices() {
                    println!("Indices found before duplication.");
                // Save indices before duplication
                let indices_copy = indices.clone();
                           // Duplicate vertices to ensure no vertices are shared
                           mesh.duplicate_vertices();
                    // Indices::set(indices.borrow_mut(), 1);
                    // wow 
                    // let indices = match indices {
                    //     Indices::U16(indices) => {println!("I use U16"); indices.iter().map(|&i| i as usize).collect::<Vec<_>>()},
                    //     Indices::U32(indices) => {println!("I use U32"); indices.iter().map(|&i| i as usize).collect::<Vec<_>>()},
                    // };

                    match indices_copy {
                        Indices::U16(indices) => mesh.insert_indices(Indices::U16(indices)),
                        Indices::U32(indices) => mesh.insert_indices(Indices::U32(indices)),
                    }

                    // Debug output after duplicating vertices
                    if mesh.indices().is_some() {
                        println!("Indices reassigned after duplication.");
                    } else {
                        println!("Failed to reassign indices after duplication.");
                        return
                    }

                    let indices = mesh.indices().unwrap();
                    let indices = match indices {
                        Indices::U16(indices) => indices.iter().map(|i| *i as usize).collect::<Vec<_>>(),
                        Indices::U32(indices) => indices.iter().map(|i| *i as usize).collect::<Vec<_>>(),
                    };

                    // Function to check if a point is inside the unit cube
                    // TODO, il manque le global transform 
                    // let pos0 = transform.transform_point(Vec3::from(positions[vertexs[0]]));
                    // let pos1 = transform.transform_point(Vec3::from(positions[vertexs[1]]));
                    // let pos2 = transform.transform_point(Vec3::from(positions[vertexs[2]]));
                    fn is_point_in_unit_cube(point: &[f32; 3]) -> bool {
                        (0.0..=1.0).contains(&point[0]) && // x
                        (0.0..=1.0).contains(&point[1]) && // y
                        (0.0..=1.0).contains(&point[2])    // z
                    }


                    let mut intersecting_triangles = Vec::new();


                    // Assign random colors to vertices
                    let mut rng = rand::thread_rng();
                    let mut colors = vec![[0.0, 0.0, 0.0, 1.0]; positions.len()];


                    // Iterate through the indices in sets of three (each triangle)
                    for vertexs in indices.chunks(3) {
                        // job 1: number of point in unit cube
                        let p1 = &positions[vertexs[0]];
                        let p2 = &positions[vertexs[1]];
                        let p3 = &positions[vertexs[2]];



                        //-------------------------------------------------------------------------------------------
                        // Create an empty mutable vector called "remember"
                        let mut remember: Vec<usize> = Vec::new();

                        // Iterate through vertexs[0], vertexs[1], and vertexs[3]
                        for &vertex in &[vertexs[0], vertexs[1], vertexs[2]] {
                            // Check if vertex is already in remember
                            if remember.contains(&vertex) {
                                println!("FOUND A DUPLICATE: {}", vertex);
                            } else {
                                // If not found, add vertex to remember
                                remember.push(vertex);

                            }
                        }
                        //-------------------------------------------------------------------------------------------





                        // does_triangle_intersect_unit_cube ? 
                        if is_point_in_unit_cube(p1) || is_point_in_unit_cube(p2) || is_point_in_unit_cube(p3) {}
                            intersecting_triangles.push((p1.clone(), p2.clone(), p3.clone()));
                        


                        // job 2: triangle color task
                        let random_color = [
                            rng.gen::<f32>(), // Red
                            rng.gen::<f32>(), // Green
                            rng.gen::<f32>(), // Blue
                            // p1[0], // Blue
                            // p1[1]/100., // Green
                            // p1[2], // Red
                            1.,            // Alpha
                        ];

                        colors[vertexs[0]] = random_color;
                        colors[vertexs[1]] = random_color;
                        colors[vertexs[2]] = random_color;
                    }

                    // job 1: number of point in unit cube
                    println!("Number of intersecting triangles: {}", intersecting_triangles.len());


                    // job 2: triangle color task
                    // Insert the color attribute into the mesh
                    // mesh.remove_attribute(Mesh::ATTRIBUTE_COLOR); // march pas 
                    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, VertexAttributeValues::Float32x4(colors));

                    
                    // job 3: make the function run once (TODO make it be CALLED until finished only)
                    commands.entity(entity).insert(Randomized);
                } else {
                    println!("No indices found in the mesh.");
                }
            } else {
                println!("No vertex positions found in the mesh.");
            } 
        } else {
            println!("PAS DE MESH");
        }
    }
}






fn bon_ta_gagne_voila_ton_update_de_merde(
    mut gizmos: Gizmos,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(&Handle<Mesh>, &GlobalTransform)>,
) {
    gizmos.cuboid(
        Transform::from_translation(Vec3::Y * 0.5).with_scale(Vec3::splat(8.)),
        Color::BLACK,
    );
    gizmos.linestrip_gradient_2d([
        (Vec2::Y * 300., Color::BLUE),
        (Vec2::new(-255., -155.), Color::RED),
        (Vec2::new(255., -155.), Color::GREEN),
        (Vec2::Y * 300., Color::BLUE),
    ]);
    for (mesh_handle, transform ) in query.iter() {
        if let Some(mesh) = meshes.get_mut(mesh_handle) {
            if let Some(VertexAttributeValues::Float32x3(positions)) = mesh.clone().attribute(Mesh::ATTRIBUTE_POSITION) {
                if let Some(indices) = mesh.clone().indices() {

                    let indices = indices.iter().map(|i| i as usize).collect::<Vec<_>>();

                    // Iterate through the indices in sets of three (each triangle)
                    for vertexs in indices.chunks(3) {
                        let pos0 = transform.transform_point(Vec3::from(positions[vertexs[0]]));
                        let pos1 = transform.transform_point(Vec3::from(positions[vertexs[1]]));
                        let pos2 = transform.transform_point(Vec3::from(positions[vertexs[2]]));

                        // Calculate the vectors for the rays
                        let vec0_to_1 = pos1 - pos0;
                        let vec1_to_2 = pos2 - pos1;
                        let vec2_to_0 = pos0 - pos2;


                        gizmos.ray_gradient(pos0, vec0_to_1, Color::BLUE, Color::RED);
                        gizmos.ray_gradient(pos1, vec1_to_2, Color::RED, Color::GREEN);
                        gizmos.ray_gradient(pos2, vec2_to_0, Color::GREEN, Color::BLUE);
                    }

                } else {
                    println!("No indices found in the mesh.");
                }
            } else {
                println!("No vertex positions found in the mesh.");
            } 
        } else {
            println!("PAS DE MESH");
        }
    }

}





