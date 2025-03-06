use bevy::{asset::RenderAssetUsages, core::Name, core_pipeline::Skybox, prelude::*, render::render_resource::{Extent3d, TextureDimension, TextureFormat}};
use bevy_rapier3d::prelude::*;

pub struct SpawnPlayerBallPlugin;

impl Plugin for SpawnPlayerBallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            // Enable this will show you gizmos of the colider, but will slow down the game
            // (slow down more with high number of polygons)
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(Startup, setup)
            ;
    }
}



fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {

    let skybox_handle = asset_server.load(super::super::skybox::CUBEMAPS[0].0); // TODO
    
    commands
        .spawn((
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("player.gltf")),),
            // Mesh3d(meshes.add(Sphere::default().mesh().uv(16, 10))),
            Visibility::default(),
            // RigidBody::KinematicPositionBased,
            // RigidBody::KinematicVelocityBased,
            Collider::capsule_y(1., 0.5),
            // Collider::ball(1.),
            Transform::from_xyz(0.0, 5.0, 0.0).with_scale(Vec3::splat(1.)),
            super::super::super::Player, // TODO fix the super super super...
            Name::new("Player"),
            // ContactSkin(0.2),
            Ccd { enabled: true },
            SoftCcd { prediction: 5.0 },
            KinematicCharacterController {
                // The translations we desire the character to move by if it doesn’t meet any obstacle.
                // translation: Option<Vect>,
                // The shape, and its position, to be used instead of the shape of the collider attached to
                // the same entity is this `KinematicCharacterController`.
                // custom_shape: Option<(Collider, Vect, Rot)>,
                // The mass to be used for impulse of dynamic bodies. This replaces the mass of the rigid-body
                // potentially associated to the collider attached to the same entity as this
                // `KinematicCharacterController`.
                //
                // This field isn’t used if `Self::apply_impulse_to_dynamic_bodies` is set to `false`.
                // custom_mass: Option<Real>,
                // The direction that goes "up". Used to determine where the floor is, and the floor’s angle.
                // up: Vect,
                // A small gap to preserve between the character and its surroundings.
                //
                // This value should not be too large to avoid visual artifacts, but shouldn’t be too small
                // (must not be zero) to improve numerical stability of the character controller.
                // offset: CharacterLength,
                // Should the character try to slide against the floor if it hits it?
                // slide: bool,
                // Should the character automatically step over small obstacles?
                // autostep: Option<CharacterAutostep>,
                // The maximum angle (radians) between the floor’s normal and the `up` vector that the
                // character is able to climb.
                // max_slope_climb_angle: Real,
                // The minimum angle (radians) between the floor’s normal and the `up` vector before the
                // character starts to slide down automatically.
                // min_slope_slide_angle: Real,
                // Should the character apply forces to dynamic bodies in its path?
                // apply_impulse_to_dynamic_bodies: bool,
                // Should the character be automatically snapped to the ground if the distance between
                // the ground and its feet are smaller than the specified threshold?
                // snap_to_ground: Some(CharacterLength::Relative(20.)),
                // Flags for filtering-out some categories of entities from the environment seen by the
                // character controller.
                // filter_flags: QueryFilterFlags,
                // Groups for filtering-out some colliders from the environment seen by the character
                // controller.
                // filter_groups: Option<CollisionGroups>,
                // Increase this number if your character appears to get stuck when sliding against surfaces.
                //
                // This is a small distance applied to the movement toward the contact normals of shapes hit
                // by the character controller. This helps shape-casting not getting stuck in an always-penetrating
                // state during the sliding calculation.
                //
                // This value should remain fairly small since it can introduce artificial "bumps" when sliding
                // along a flat surface.
                // normal_nudge_factor: Real,

                // offset: CharacterLength::Relative(0.5),
                // snap_to_ground: Some(CharacterLength::Absolute(0.5)),
                
                ..KinematicCharacterController::default()
            },
        ))
        .with_child((
            // Transform::from_xyz(1., 1., 5.).looking_at(Vec3::from_array([1., -55., 5.]), Vec3::Y),
            Transform::from_xyz(1., 1., 5.),
            Camera3d {
                ..default()
            },
            Skybox {
                image: skybox_handle.clone(),
                brightness: 1000.0,
                rotation:  Quat::IDENTITY,
            },
        ))
        ;
}


// try to reach animation.................
// https://bevyengine.org/examples/animation/animated-fox/ dont help