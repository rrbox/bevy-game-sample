use bevy::app::Plugin;

use crate::game::{
    conversation::{
        self, conversation_data::create_conversation, domain_models::{Flow, Passage, PassageID, Scenario}, events::DisplayPassageEvent, static_resources::Conversation, ui::ConversationUiPlugin
    },
    shared::domain_models::conversation::{
        scenario_ids::ScenarioIDs, start_conversation::StartConversationEvent,
    },
};

mod conversation_data;
mod domain_models;
pub mod events;
mod static_resources;
pub mod ui;

pub struct ConversationPlugin;

impl Plugin for ConversationPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<StartConversationEvent>()
            .add_event::<DisplayPassageEvent>()
            .insert_resource(create_conversation())
            .add_plugins(ConversationUiPlugin);
    }
}
