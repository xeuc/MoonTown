
use bevy::prelude::*;

#[derive(Component)]
pub struct Anchor;


pub const GROUND_TIMER: f32 = 0.5;
pub const MOVEMENT_SPEED: f32 = 8.0;
pub const JUMP_SPEED: f32 = 20.0;
pub const GRAVITY: f32 = -9.81;
