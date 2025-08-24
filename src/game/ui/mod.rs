use crate::game::states::AppState;
use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod systems;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::setup_conversation_ui)
            .add_systems(
                Update,
                (
                    systems::close_conversation_ui_system,
                    systems::handle_start_conversation_event_system,
                    systems::toggle_pause_system, // <-- この行を追加
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}
