use bevy::app::{PluginGroup, PluginGroupBuilder};

mod player;
use crate::game_plugins::player::PlayerPlugin;

pub struct GamePlugins;
impl PluginGroup for GamePlugins {
    
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PlayerPlugin)
    }
}
