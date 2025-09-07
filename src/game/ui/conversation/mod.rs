use bevy::prelude::*;

use crate::game::states::{GameState, PauseState};

pub mod components;
pub mod events;
pub mod systems;

pub struct ConversationUiPlugin;

impl Plugin for ConversationUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::setup_conversation_ui)
            .add_systems(
                Update,
                (
                    systems::close_conversation_ui_system,
                    systems::handle_start_conversation_event_system,
                )
                    .run_if(in_state(GameState::Conversation))
                    .run_if(in_state(PauseState::Running)),
            );
    }
}
