use bevy::app::{PluginGroup, PluginGroupBuilder};

mod player;
use crate::game_plugins::player::PlayerPlugin;
mod world;
use crate::game_plugins::world::WorldPlugin;
mod ui;
use crate::game_plugins::ui::UIPlugin;
mod inputs; 
use crate::game_plugins::inputs::InputsPlugin;
mod camera; 
use crate::game_plugins::camera::CameraPlugin;
mod shared; 


pub struct GamePlugins;
impl PluginGroup for GamePlugins {
    
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PlayerPlugin)
            .add(WorldPlugin)
            .add(UIPlugin)
            .add(InputsPlugin)
            .add(CameraPlugin)
    }
}
