use bevy::app::{PluginGroup, PluginGroupBuilder};


mod player;  use player::PlayerPlugin;
mod world;   use world::WorldPlugin;
mod ui;      use ui::UIPlugin;
mod inputs;  use inputs::InputsPlugin;
mod camera;  use camera::CameraPlugin;
mod shared; 


pub struct GamePlugins;
impl PluginGroup for GamePlugins {
    
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CameraPlugin)
            .add(InputsPlugin)
            .add(PlayerPlugin)
            .add(UIPlugin)
            .add(WorldPlugin)
    }
}
