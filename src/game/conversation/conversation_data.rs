use crate::game::{
    conversation::{
        domain_models::{Flow, Passage, Scenario},
        static_resources::Conversation,
    },
    shared::domain_models::conversation::scenario_ids::ScenarioIDs,
};

pub(crate) fn create_conversation() -> Conversation {
    Conversation::new(vec![(
        ScenarioIDs::section1_start(),
        Scenario::new(
            0.into(),
            vec![
                (
                    0.into(),
                    Passage::new("NPC", "こんにちは！", Flow::linear(1.into())),
                ),
                (
                    1.into(),
                    Passage::new(
                        "NPC",
                        "ここのダンジョンは危険だ...！",
                        Flow::linear(2.into()),
                    ),
                ),
                (
                    2.into(),
                    Passage::new("NPC", "気をつけて...！", Flow::end(0.into())),
                ),
            ],
        ),
    )])
}
