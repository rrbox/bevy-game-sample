use bevy::prelude::*;
mod systems;

pub struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::force_ingame_state_system);
    }
}
