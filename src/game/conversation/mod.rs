use bevy::app::Plugin;

use crate::game::{
    conversation::{
        domain_models::{Flow, Passage, PassageID, Scenario},
        static_resources::Conversation,
        ui::ConversationUiPlugin,
    },
    shared::domain_models::conversation::scenario_ids::ScenarioIDs,
};

mod domain_models;
mod static_resources;
pub mod ui;

pub struct ConversationPlugin;

impl Plugin for ConversationPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(Conversation::new(vec![(
            ScenarioIDs::start(),
            Scenario::new(
                PassageID::new(0),
                vec![(
                    PassageID::new(0),
                    Passage::new("NPC_A", "こんにちは！", Flow::end(0.into())),
                )],
            ),
        )]))
        .add_plugins(ConversationUiPlugin);
    }
}
