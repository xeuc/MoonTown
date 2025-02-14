//! A basic implementation of a character controller for a dynamic rigid body.
//!
//! This showcases the following:
//!
//! - Basic directional movement and jumping
//! - Support for both keyboard and gamepad input
//! - A configurable maximum slope angle for jumping
//! - Loading a platformer environment from a glTF
//!
//! The character controller logic is contained within the `plugin` module.
//!
//! For a kinematic character controller, see the `kinematic_character_3d` example.

use bevy::{
    core_pipeline::Skybox,
    image::CompressedImageFormats,
    prelude::*,
    render::{
        render_resource::{TextureViewDescriptor, TextureViewDimension},
        renderer::RenderDevice,
    },
};

pub struct SkyboxPlugin;

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup_skybox)
        .add_systems(
            Update,
            (
                cycle_cubemap_asset,
                asset_loaded.after(cycle_cubemap_asset),
            ),
        );
    }
}


#[derive(Resource)]
pub struct Cubemap {is_loaded: bool, index: usize, image_handle: Handle<Image>,}

pub const CUBEMAPS: &[(&str, CompressedImageFormats)] = &[
    ("textures/cubemaps_office.png", CompressedImageFormats::NONE,),
    // ("textures/Ryfjallet_cubemap_bc7.ktx2", CompressedImageFormats::BC,),
];

const CUBEMAP_SWAP_DELAY: f32 = 3.0;



fn setup_skybox(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let skybox_handle = assets.load(CUBEMAPS[0].0);
    commands.insert_resource(Cubemap {
        is_loaded: false,
        index: 0,
        image_handle: skybox_handle,
    });
}

fn cycle_cubemap_asset(
    time: Res<Time>,
    mut next_swap: Local<f32>,
    mut cubemap: ResMut<Cubemap>,
    asset_server: Res<AssetServer>,
    render_device: Res<RenderDevice>,
) {
    let now = time.elapsed_secs();
    if *next_swap == 0.0 {
        *next_swap = now + CUBEMAP_SWAP_DELAY;
        return;
    } else if now < *next_swap {
        return;
    }
    *next_swap += CUBEMAP_SWAP_DELAY;

    let supported_compressed_formats =
        CompressedImageFormats::from_features(render_device.features());

    let mut new_index = cubemap.index;
    for _ in 0..CUBEMAPS.len() {
        new_index = (new_index + 1) % CUBEMAPS.len();
        if supported_compressed_formats.contains(CUBEMAPS[new_index].1) {
            break;
        }
        info!("Skipping unsupported format: {:?}", CUBEMAPS[new_index]);
    }

    // Skip swapping to the same texture. Useful for when ktx2, zstd, or compressed texture support
    // is missing
    if new_index == cubemap.index {
        return;
    }

    cubemap.index = new_index;
    cubemap.image_handle = asset_server.load(CUBEMAPS[cubemap.index].0);
    cubemap.is_loaded = false;
}







fn asset_loaded(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
    mut skyboxes: Query<&mut Skybox>,
) {
    if !cubemap.is_loaded && asset_server.load_state(&cubemap.image_handle).is_loaded() {
        // info!("Swapping to {}...", CUBEMAPS[cubemap.index].0); // TODO
        let image = images.get_mut(&cubemap.image_handle).unwrap();
        // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
        // so they appear as one texture. The following code reconfigures the texture as necessary.
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(image.height() / image.width());
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }

        for mut skybox in &mut skyboxes {
            skybox.image = cubemap.image_handle.clone();
        }

        cubemap.is_loaded = true;
    }
}
