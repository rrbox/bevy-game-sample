use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, systems::setup_conversation_ui)
            .add_systems(Update, systems::toggle_conversation_ui);
    }
}
