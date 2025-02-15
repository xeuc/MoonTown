use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const SPEED: f32 = 10.0;
const JUMP_FORCE: f32 = 10.0;
const GRAVITY: f32 = -9.81;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec3);

pub struct SpawnTerrainPlugin;

impl Plugin for SpawnTerrainPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default())
            // .insert_resource(GravityScale(GRAVITY))
            .add_systems(Startup, setup)
            .add_systems(Update, player_movement)
            .add_systems(Startup, setup_physics)
            .add_systems(Update, read_result_system);
    }
}


fn setup_physics(mut commands: Commands) {
    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(0.5))
        .insert(Transform::default())
        .insert(KinematicCharacterController {
            ..KinematicCharacterController::default()
        });
}

fn read_result_system(controllers: Query<(Entity, &KinematicCharacterControllerOutput)>) {
    for (entity, output) in controllers.iter() {
        println!(
            "Entity {:?} moved by {:?} and touches the ground: {:?}",
            entity, output.effective_translation, output.grounded
        );
    }
}


fn setup(mut commands: Commands) {
    commands.spawn((
        KinematicCharacterController {
            ..KinematicCharacterController::default()
        },
        Player,
        Velocity(Vec3::ZERO),
        RigidBody::Dynamic,
        Collider::capsule_y(1.8, 1.0),
        ActiveEvents::COLLISION_EVENTS,
        LockedAxes::ROTATION_LOCKED,
        
    ));

    commands
        .spawn(Collider::cuboid(200.0, 0.2, 200.0))
        .insert(Transform::from_xyz(0.0, -4.0, 0.0));

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(1.))
        .insert(Restitution::coefficient(1.4))
        .insert(Transform::from_xyz(0.0, 8.0, 0.0));
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