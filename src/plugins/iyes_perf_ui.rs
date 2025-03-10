use bevy::prelude::*;
use iyes_perf_ui::{entries::PerfUiAllEntries, prelude::PerfUiDefaultEntries, PerfUiPlugin};

pub struct IvesPerfUIPlugin;

impl Plugin for IvesPerfUIPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(PerfUiPlugin)
        .add_systems(Startup, setup_debug);
    }
}


fn setup_debug(
    mut commands: Commands,
) {
    commands.spawn(PerfUiAllEntries::default());
}

