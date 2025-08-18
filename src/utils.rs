
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, input::{mouse::MouseMotion, InputSystem}, prelude::*
};
use bevy_rapier3d::{control::KinematicCharacterController, prelude::*};
use bevy::{
    render::camera::Viewport, window::WindowResized,
};


/// Keyboard input vector
#[derive(Default, Resource, Deref, DerefMut)]
pub struct MovementInput(Vec3);

/// Mouse input vector
#[derive(Default, Resource, Deref, DerefMut)]
pub struct LookInput(Vec2);


#[derive(Component)]
pub struct CameraPosition {
    pub pos: UVec2,
}
#[derive(Component)]
pub struct RotateCamera(pub Direction);

#[derive(Component)]
pub struct TopLeftCamera;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct Player;

pub enum Direction {
    Left,
    Right,
}

pub const MOUSE_SENSITIVITY: f32 = 0.3;
pub const GROUND_TIMER: f32 = 0.5;
pub const MOVEMENT_SPEED: f32 = 8.0;
pub const JUMP_SPEED: f32 = 20.0;
pub const GRAVITY: f32 = -9.81;
