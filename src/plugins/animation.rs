//! Plays animations from a skinned glTF.


use std::time::Duration;

use bevy::{
    animation::RepeatAnimation,
    prelude::*,
};

const PLAYER_MODEL_PATH: &str = "avatar_model_test_16x16_scale16.gltf";


pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, setup_scene_once_loaded)
            .add_systems(Update, keyboard_animation_control)
            ;
    }
}

#[derive(Resource)]
pub struct Animations {
    pub animations: Vec<AnimationNodeIndex>,
    graph: Handle<AnimationGraph>,
}

#[derive(Event, Reflect, Clone)]
struct OnStep;


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    // Build the animation graph
    let (graph, node_indices) = AnimationGraph::from_clips([
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(PLAYER_MODEL_PATH)), // idle
        asset_server.load(GltfAssetLabel::Animation(1).from_asset(PLAYER_MODEL_PATH)), // jump
        asset_server.load(GltfAssetLabel::Animation(2).from_asset(PLAYER_MODEL_PATH)), // zzz
        asset_server.load(GltfAssetLabel::Animation(3).from_asset(PLAYER_MODEL_PATH)), // running
    ]);

    // Insert a resource with the current scene information
    let graph_handle = graphs.add(graph);
    commands.insert_resource(Animations {
        animations: node_indices,
        graph: graph_handle,
    });
}

// An `AnimationPlayer` is automatically added to the scene when it's ready.
// When the player is added, start the animation.
fn setup_scene_once_loaded(
    mut commands: Commands,
    animations: Res<Animations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {


    for (entity, mut player) in &mut players {

        let mut transitions = AnimationTransitions::new();

        // Make sure to start the animation via the `AnimationTransitions`
        // component. The `AnimationTransitions` component wants to manage all
        // the animations and will get confused if the animations are started
        // directly via the `AnimationPlayer`.
        transitions
            .play(&mut player, animations.animations[0], Duration::ZERO)
            .repeat();

        commands
            .entity(entity)
            .insert(AnimationGraphHandle(animations.graph.clone()))
            .insert(transitions);
    }
}































fn keyboard_animation_control(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    animations: Res<Animations>,
    mut current_animation: Local<usize>,
) {
    for (mut player, mut transitions) in &mut animation_players {
        let Some((&playing_animation_index, _)) = player.playing_animations().next() else {
            continue;
        };

        // play / pause
        if keyboard_input.just_pressed(KeyCode::KeyP) {
            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
            if playing_animation.is_paused() {
                playing_animation.resume();
            } else {
                playing_animation.pause();
            }
        }

        // speed up animation playback
        if keyboard_input.just_pressed(KeyCode::KeyP) {
            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
            let speed = playing_animation.speed();
            playing_animation.set_speed(speed * 1.2);
        }

        // slow down animation playback
        if keyboard_input.just_pressed(KeyCode::KeyP) {
            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
            let speed = playing_animation.speed();
            playing_animation.set_speed(speed * 0.8);
        }

        // seek backward
        if keyboard_input.just_pressed(KeyCode::KeyP) {
            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
            let elapsed = playing_animation.seek_time();
            playing_animation.seek_to(elapsed - 0.1);
        }

        // seek forward
        if keyboard_input.just_pressed(KeyCode::KeyP) {
            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
            let elapsed = playing_animation.seek_time();
            playing_animation.seek_to(elapsed + 0.1);
        }

        // change animation
        if keyboard_input.just_pressed(KeyCode::KeyE) {
            *current_animation = (*current_animation + 1) % animations.animations.len();

            transitions
                .play(
                    &mut player,
                    animations.animations[*current_animation],
                    Duration::from_millis(250),
                )
                .repeat();
        }

        // digit 1 / 3 / 5: play the animation <digit> times
        if keyboard_input.just_pressed(KeyCode::KeyP) {
            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
            playing_animation
                .set_repeat(RepeatAnimation::Count(1))
                .replay();
        }

        // digit 1 / 3 / 5: play the animation <digit> times
        if keyboard_input.just_pressed(KeyCode::KeyP) {
            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
            playing_animation
                .set_repeat(RepeatAnimation::Count(3))
                .replay();
        }

        // digit 1 / 3 / 5: play the animation <digit> times
        if keyboard_input.just_pressed(KeyCode::KeyP) {
            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
            playing_animation
                .set_repeat(RepeatAnimation::Count(5))
                .replay();
        }

        // loop the animation forever
        if keyboard_input.just_pressed(KeyCode::KeyP) {
            let playing_animation = player.animation_mut(playing_animation_index).unwrap();
            playing_animation.set_repeat(RepeatAnimation::Forever);
        }
    }
}