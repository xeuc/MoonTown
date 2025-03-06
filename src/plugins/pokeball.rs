use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct SpawnPokeBallPlugin;

impl Plugin for SpawnPokeBallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_pokeball)
            ;
    }
}



fn setup_pokeball(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Spawn the map
    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("pokeball.gltf")),),
        RigidBody::Dynamic,
        Collider::ball(2.),
        Restitution::coefficient(0.7),
        GravityScale(1.),
        Transform::from_xyz(10.,2., 10.).with_scale(Vec3::splat(1.)),
        // AsyncSceneCollider::default(),
    ));
}

