
use bevy::prelude::*;



pub mod systems;
use crate::game_plugins::world::systems::*;


pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                setup_maps_elements,
            ))
        ;
    }
}

