use bevy::prelude::*;

use crate::game::{
    states::{GameState, PauseState},
};

pub mod components;
pub mod systems;

pub struct ConversationUiPlugin;

impl Plugin for ConversationUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::setup_conversation_ui)
            .add_systems(
                Update,
                // This system can run in any state, as it listens for an event
                systems::handle_start_conversation_event_system
                    .run_if(in_state(PauseState::Running)),
            )
            .add_systems(
                Update,
                (
                    systems::advance_conversation_system,
                    systems::display_passage_system,
                )
                    .chain()
                    .run_if(in_state(GameState::Conversation))
                    .run_if(in_state(PauseState::Running)),
            );
    }
}
