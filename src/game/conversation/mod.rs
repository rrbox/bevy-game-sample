use bevy::app::Plugin;

use crate::game::{
    conversation::{
        domain_models::{Flow, Passage, PassageID, Scenario},
        events::DisplayPassageEvent,
        static_resources::Conversation,
        ui::ConversationUiPlugin,
    },
    shared::domain_models::{
        conversation::scenario_ids::ScenarioIDs,
        conversation::start_conversation::StartConversationEvent,
    },
};

mod domain_models;
pub mod events;
mod static_resources;
pub mod ui;

pub struct ConversationPlugin;

impl Plugin for ConversationPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<StartConversationEvent>()
            .add_event::<DisplayPassageEvent>()
            .insert_resource(Conversation::new(vec![(
                ScenarioIDs::start(),
                Scenario::new(
                    PassageID::new(0),
                    vec![
                        (
                            PassageID::new(0),
                            Passage::new("NPC_A", "Hello, adventurer.", Flow::linear(1.into())),
                        ),
                        (
                            PassageID::new(1),
                            Passage::new(
                                "NPC_A",
                                "This dungeon is dangerous. Be careful.",
                                Flow::end(0.into()),
                            ),
                        ),
                    ],
                ),
            )]))
            .add_plugins(ConversationUiPlugin);
    }
}
