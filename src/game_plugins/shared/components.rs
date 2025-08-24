use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct Player;


/// Mouse input vector
#[derive(Default, Resource, Deref, DerefMut)]
pub struct LookInput(Vec2);

/// Keyboard input vector
#[derive(Default, Resource, Deref, DerefMut)]
pub struct MovementInput(Vec3);





pub const MOUSE_SENSITIVITY: f32 = 0.3;
