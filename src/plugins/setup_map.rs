use std::borrow::BorrowMut;
use std::{cmp, default};

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



                    // while being in the setup of the foreach triangles of the map
                    // get the chunks 
                    // 1 find the Xmin Xmax Ymin Ymax Zmin Zmax coordinate and you have your BIG box chunk

                    // set len of chunck
                    let h:u8 = 1;

                    // make tri pos more readable
                    let x0 = position_time_three[0][0]; let x1 = position_time_three[1][0]; let x2 = position_time_three[2][0];
                    let y0 = position_time_three[0][1]; let y1 = position_time_three[1][1]; let y2 = position_time_three[2][1];
                    let z0 = position_time_three[0][2]; let z1 = position_time_three[1][2]; let z2 = position_time_three[2][2];

                    // determine bounding box coord
                    let x_min = x0.min(x1.min(x2)); let x_max = x0.max(x1.max(x2));
                    let y_min = y0.min(y1.min(y2)); let y_max = y0.max(y1.max(y2));
                    let z_min = z0.min(z1.min(z2)); let z_max = z0.max(z1.max(z2));

                    // test if triangle is ENTIRELY inside chunk (I hope)
                    if x_min == x_max && y_min == y_max && z_min == z_max{
                        // yes
                        // TODO Add this triangle in chunk table bizz
                    }

                    fn get_chunk_coordinate(f: f32, h: u8) -> u8 {
                        return (f/h as f32).floor() as u8;
                    }
                    
                    let xchunk_min = get_chunk_coordinate(x_min, h); let xchunk_max = get_chunk_coordinate(x_max, h);
                    let ychunk_min = get_chunk_coordinate(y_min, h); let ychunk_max = get_chunk_coordinate(y_max, h);
                    let zchunk_min = get_chunk_coordinate(z_min, h); let zchunk_max = get_chunk_coordinate(z_max, h);

                    // TODO is ychunk_max = ychunk_max or ychunk_max = ychunk_max + h ? 

                    // TODO erase nested code

                    fn get_plane_equation(
                        x0: f32, y0: f32, z0: f32,
                        x1: f32, y1: f32, z1: f32,
                        x2: f32, y2: f32, z2: f32,
                        // x: f32, y: f32, z: f32,
                    ) -> (f32, f32, f32, f32) {
                        // Plan of the triangle
                        let x10 = x1 - x0; let x20 = x2 - x0;
                        let y10 = y1 - y0; let y20 = y2 - y0;
                        let z10 = z1 - z0; let z20 = z2 - z0;
                        // n = |   x   y   z |
                        //     | x10 y10 z10 |
                        //     | x20 y20 z20 |
                        // let n = (y10*z20 - y20*z10)*x - (x10*z20 - x20*z10)*y + (x10*y20 - x20*y10)*z;
                        let a = (y10*z20 - y20*z10); // TODO naming's rude man
                        let b = - (x10*z20 - x20*z10);
                        let c = (x10*y20 - x20*y10);
                        // Plan is Ax + By + Cz = D
                        let d = a*x0 + b*y0 + c*z0;
                        return (a, b, c, d);

                    }

                    let (a, b, c, d) = get_plane_equation(x0, y0, z0, x1, y1, z1, x2, y2, z2);

                    #[derive(Clone, Copy, PartialEq)]  // Assuming this trait is needed
                    enum WhereIsThePointRegardingTheTriangle {
                        OnTopOf,
                        IntersectingIeInside,
                        Below,
                        Unknown,
                    }
                    // TODO do not recalculate the plane equasion each time
                    fn where_is_the_point_regarding_the_triangle(
                        a: f32, b: f32, c: f32, d: f32,
                        x: f32, y: f32, z: f32,
                    ) -> WhereIsThePointRegardingTheTriangle {
                        // Epsilon value for floating-point comparison
                        let e = 1e-6;

                        let naming_var = a*x + b*y + c*z - d;
                        if naming_var > e {
                            return WhereIsThePointRegardingTheTriangle::OnTopOf;
                        } else if naming_var < -e {
                            return WhereIsThePointRegardingTheTriangle::Below;
                        } else {
                            return WhereIsThePointRegardingTheTriangle::IntersectingIeInside;
                        }
                    }

                    // let mut where_is_the_point_regarding_the_triangle_sav: [[[WhereIsThePointRegardingTheTriangle; xchunk_max - xchunk_min + 1]; ychunk_max - ychunk_min + 1]; zchunk_max - zchunk_min + 1] ;
                    let mut where_is_the_point_regarding_the_triangle_sav: Vec<Vec<Vec<WhereIsThePointRegardingTheTriangle>>> = vec![vec![vec![WhereIsThePointRegardingTheTriangle::Unknown; (xchunk_max - xchunk_min + 1) as usize]; (ychunk_max - ychunk_min + 1) as usize]; (zchunk_max - zchunk_min + 1) as usize]; // TODO Line a bit too long
                    
                    // for each chunks containing (?) the triangle
                    let mut is_the_chunk_sliced_by_the_triangle: bool = false; // #estimation
                    for x in (xchunk_min..xchunk_max).step_by(h as usize) {
                        for y in (ychunk_min..ychunk_max).step_by(h as usize) {
                            for z in (zchunk_min..zchunk_max).step_by(h as usize) {
                                let x = x as usize; let y = y as usize; let z = z as usize; let h = h as usize;
                                
                                
                                
                                fn fill_tab_with_pos_point(
                                    where_is_the_point_regarding_the_triangle_sav: &mut Vec<Vec<Vec<WhereIsThePointRegardingTheTriangle>>>,
                                    x: usize, y: usize, z: usize, 
                                    a: f32, b: f32, c: f32, d: f32,
                                ) {
                                    // Is the pos of the point regarding the triangle is not already calculated? 
                                    if where_is_the_point_regarding_the_triangle_sav[x][y][z] == WhereIsThePointRegardingTheTriangle::Unknown {
                                        let where_is_tri = where_is_the_point_regarding_the_triangle(a, b, c, d, x as f32, y as f32, z as f32);
                                        where_is_the_point_regarding_the_triangle_sav[x][y][z] = where_is_tri;
                                    }
                                }

                                let xh: usize = (x + h) as usize; let yh: usize = (y + h) as usize; let zh: usize = (z + h) as usize; // temp values
                                let points_to_test = [
                                    [x , y , z ],
                                    [x , y , zh],
                                    [x , yh, z ],
                                    [x , yh, zh],
                                    [xh, y , z ],
                                    [xh, y , zh],
                                    [xh, yh, z ],
                                    [xh, yh, zh],
                                ];

                                for triple_of_points in points_to_test {
                                    fill_tab_with_pos_point(&mut where_is_the_point_regarding_the_triangle_sav, triple_of_points[0], triple_of_points[1], triple_of_points[2], a, b, c, d);
                                }

                                // Is the cube be sliced by the triangle
                                let i_guess_the_pos_of_hte_point_is = where_is_the_point_regarding_the_triangle_sav[x][y][z];

                                for triple_of_points in points_to_test { // TODO Don't check the first one twice.. 
                                    if where_is_the_point_regarding_the_triangle_sav[triple_of_points[0]][triple_of_points[1]][triple_of_points[2]] != i_guess_the_pos_of_hte_point_is {
                                        // YES THE TRIANGLE SLICE (or is contained in) the chunck
                                        is_the_chunk_sliced_by_the_triangle = true;
                                    }
                                    break;
                                }


                            }
                        }
                    }

                    if is_the_chunk_sliced_by_the_triangle {
                        // TODO: PUT IN VECT
                    }

                    // STEP 2 DIAGONALS 



                    // but some of them are empty.. 
                    // apres pour savoir le la ligne traverse le cube il faut aller en 2D:
                    // foreach xa


                    
                    // 2D
                    // https://stackoverflow.com/questions/2049582/how-to-determine-if-a-point-is-in-a-2d-triangle
                    fn sign(
                        xp1: f32, yp1: f32,
                        xp2: f32, yp2: f32,
                        xp3: f32, yp3: f32,
                    ) -> f32 {
                        return (xp1 - xp3) * (yp2 - yp3) - (xp2 - xp3) * (yp1 - yp3);
                    }

                    fn point_in_triangle(
                        xpt: f32, ypt: f32, zpt: f32,
                        xv1: f32, yv1: f32, zv1: f32,
                        xv2: f32, yv2: f32, zv2: f32,
                        xv3: f32, yv3: f32, zv3: f32,
                    ) -> bool {
                        let d1 = sign(
                            xpt, ypt,
                            xv1, yv1,
                            xv2, yv2,
                        );
                        let d2 = sign(
                            xpt, ypt,
                            xv2, yv2,
                            xv3, yv3,
                        );
                        let d3 = sign(
                            xpt, ypt,
                            xv3, yv3,
                            xv1, yv1,
                        );

                        let has_neg = (d1 < 0.) || (d2 < 0.) || (d3 < 0.);
                        let has_pos = (d1 > 0.) || (d2 > 0.) || (d3 > 0.);

                        return !(has_neg && has_pos);
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





