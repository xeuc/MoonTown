use bevy::prelude::*;
use bevy_rapier3d::geometry::*;
// use bevy_rapier3d::prelude::SoftCcd;

pub struct SetupMapPlugin;

impl Plugin for SetupMapPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup_map)
        ;
    }
}

// Setup System
fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Spawn the map
    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("creative_map_simple3.gltf")),),
        // ContactSkin(0.2),
        // SoftCcd { prediction: 200. },
        AsyncSceneCollider::default(),
    ));
}
