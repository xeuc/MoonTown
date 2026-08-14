use bevy::prelude::*;


#[derive(Component)]
pub struct TopLeftCamera;

#[derive(Component)]
pub struct CameraViewUIPositionOnScreen {
    pub pos: UVec2,
}


#[derive(Component)]
pub struct RotateCamera(pub Direction);

pub enum Direction {
    Left,
    Right,
}