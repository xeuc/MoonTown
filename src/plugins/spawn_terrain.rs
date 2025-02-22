use bevy::{asset::RenderAssetUsages, prelude::*, render::render_resource::{Extent3d, TextureDimension, TextureFormat}};
use bevy_rapier3d::prelude::*;

const SPEED: f32 = 10.0;
const JUMP_FORCE: f32 = 10.0;
const GRAVITY: f32 = -9.81;

pub struct SpawnTerrainPlugin;

impl Plugin for SpawnTerrainPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            // .add_plugins(RapierDebugRenderPlugin::default())
            // .insert_resource(GravityScale(GRAVITY))
            .add_systems(Startup, setup)
            .add_systems(Update, player_movement)
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
        .insert(Transform::from_xyz(0.0, 200.0, 0.0))
        .insert(KinematicCharacterController {
            ..KinematicCharacterController::default()
        });

    // commands.spawn((
    //     KinematicCharacterController {
    //         ..KinematicCharacterController::default()
    //     },
    //     Player,
    //     RigidBody::Dynamic,
    //     Collider::capsule_y(1.8, 1.0),
    //     ActiveEvents::COLLISION_EVENTS,
    //     LockedAxes::ROTATION_LOCKED,
        
    // ));

    // commands
    //     .spawn(Collider::cuboid(200.0, 0.2, 200.0))
    //     .insert(Transform::from_xyz(0.0, -4.0, 0.0));

    // commands
    //     .spawn(RigidBody::Dynamic)
    //     .insert(Collider::ball(1.))
    //     .insert(Restitution::coefficient(1.4))
    //     .insert(Transform::from_xyz(0.0, 8.0, 0.0));
}


fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut controllers: Query<&mut KinematicCharacterController>,
) {
    for mut controller in controllers.iter_mut() {
    // if let Ok(mut controller) = controllers.get_single_mut() {
        let mut movement = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Numpad8) {
            movement.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Numpad5) {
            movement.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Numpad4) {
            movement.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Numpad6) {
            movement.x += 1.0;
        }

        movement = movement.normalize_or_zero() * SPEED * time.delta_secs();

        if keyboard_input.just_pressed(KeyCode::Numpad7) {
            movement.y = JUMP_FORCE;
        }

        movement.y += GRAVITY * time.delta_secs();
        controller.translation = Some(movement);


        // controller.translation = Some(Vec3::new(1.0, -5.0, -1.0) * time.delta_secs());
    }
}