use std::borrow::BorrowMut;
use std::default;

use bevy::prelude::*;
use bevy::render::mesh::{Indices, VertexAttributeValues};

use bevy::asset::{AssetServer, Assets, Handle, LoadState};
use bevy::core_pipeline::Skybox;

use rand::*;

#[derive(Default)]
struct Player {
    entity: Option<Entity>,
    // i: usize,
    // j: usize,
    // move_cooldown: Timer,
}
#[derive(Resource, Default)]
struct Game {
    // board: Vec<Vec<Cell>>,
    player: Player,
    map_triangle_colors: Vec<[f32; 4]>,
    // bonus: Bonus,
    // score: i32,
    // cake_eaten: u32,
    // camera_should_focus: Vec3,
    // camera_is_focus: Vec3,
}

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
        .init_resource::<Game>()
        .add_systems(Startup, setup2)
        .add_systems(Startup, setup)

        // .add_systems(Startup, check_asset_load_state.after(setup2))
        
        // .add_systems(Update, make_bevy_wait_bc_he_does_not_know_how)
        // .add_systems(Update, check_mesh_ready_no_rapier.after(make_bevy_wait_bc_he_does_not_know_how))

        .add_systems(Update, check_mesh_ready_no_rapier)
        .add_systems(Update, bon_ta_gagne_voila_ton_update_de_merde)
        ;
    }
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut game: ResMut<Game>,
) {
    // Player
    let skybox_handle = assets.load(super::skybox::CUBEMAPS[0].0); // TODO
    let entity_player = commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 100., 0.0),
            ..default()
        },
        Skybox {
            image: skybox_handle.clone(),
            brightness: 1000.0,
        },
    ))
    .id()
    ;

    game.player.entity = Some(entity_player);
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
    mut game: ResMut<Game>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Handle<Mesh>, &Handle<StandardMaterial>, &GlobalTransform), Without<Randomized>>,
) {
    // println!("START FUNCTION check_mesh_ready_no_rapier");
    for (entity, mesh_handle, material_handle, global_transform) in query.iter() {
        if let Some(standard_material) = materials.get_mut(material_handle) {
            standard_material.base_color = Color::rgb(1.0, 1.0, 1.0); // WHITE
        }

        if let Some(mut mesh) = meshes.get_mut(mesh_handle) {
            mesh.duplicate_vertices();
            // WHY THERE NO INDICE ANYMORE IN MESH? 
            // NO INDICES
            // YOU MAY NOT NEED THEM THOUGH, BUT IT's CLEARLY A BEVY MISSed FEATURE
 
            if let Some(VertexAttributeValues::Float32x3(positions)) = mesh.clone().attribute(Mesh::ATTRIBUTE_POSITION) {

                let mut rng = rand::thread_rng();
                let mut colors = vec![[0.0, 0.0, 0.0, 1.0]; positions.len()];
                let mut counter = 0;
                for position_time_three in positions.chunks(3) {
                    let random_color = [
                        rng.gen::<f32>(), // Red
                        rng.gen::<f32>(), // Green
                        rng.gen::<f32>(), // Blue
                        1.,            // Alpha
                    ];

                    for _ in 0..3 {
                        colors[counter] = random_color;
                        counter += 1;
                    }

                }
                mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, VertexAttributeValues::Float32x4(colors.clone()));
                game.map_triangle_colors = colors;
                commands.entity(entity).insert(Randomized);

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
    mut game: ResMut<Game>,
    mut commands: Commands,
    // mut world: ResMut<World>,
    mut transforms: Query<&mut Transform>,
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

                // let mut colors = match mesh.attribute(Mesh::ATTRIBUTE_COLOR) {
                //     Some(VertexAttributeValues::Float32x4(colors2)) => colors2.clone(),
                //     _ => {
                //         println!("Error occurs on colors of the mesh. Id = 524");
                //         return;
                //     },
                // };
                let mut colors = game.map_triangle_colors.clone();

                let mut counter = 0;

                for position_time_three in positions.chunks(3) {

                    // TODO: make sure you don't redraw already drawn gizmo
                    let pos0 = transform.transform_point(Vec3::from(position_time_three[0]));
                    let pos1 = transform.transform_point(Vec3::from(position_time_three[1]));
                    let pos2 = transform.transform_point(Vec3::from(position_time_three[2]));
                    
                    let vec0_to_1 = pos1 - pos0;
                    let vec1_to_2 = pos2 - pos1;
                    let vec2_to_0 = pos0 - pos2;

                    gizmos.ray_gradient(pos0, vec0_to_1, Color::BLUE, Color::RED);
                    gizmos.ray_gradient(pos1, vec1_to_2, Color::BLUE, Color::RED);
                    gizmos.ray_gradient(pos2, vec2_to_0, Color::BLUE, Color::RED);


                    let aze = *transforms.get_mut(game.player.entity.unwrap()).unwrap();
                    let coordinate = aze.translation;
                    for pos in [pos0, pos1, pos2] {
                        if 10000. > (pos[0] - coordinate[0])*(pos[0] - coordinate[0]) + (pos[1] - coordinate[1])*(pos[1] - coordinate[1]) + (pos[2] - coordinate[2])*(pos[2] - coordinate[2]) {
                            colors[counter + 0] = [1., 0., 0., 1.];
                            colors[counter + 1] = [1., 0., 0., 1.];
                            colors[counter + 2] = [1., 0., 0., 1.];
                        }
                    }
                    counter += 3;

                }
                mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, VertexAttributeValues::Float32x4(colors.clone()));

            } else {
                println!("No vertex positions found in the mesh.");
            } 
        } else {
            println!("PAS DE MESH");
        }
    }

}





