use crate::game::{
    conversation::domain_models::Scenario,
    shared::domain_models::conversation::scenario_ids::ScenarioID,
};
use bevy::{prelude::*, utils::HashMap};

#[derive(Resource)]
pub struct Conversation {
    pub(crate) scenarios: HashMap<ScenarioID, Scenario>,
}

impl Conversation {
    pub fn new(scenarios: Vec<(ScenarioID, Scenario)>) -> Self {
        Conversation {
            scenarios: scenarios.into_iter().collect(),
        }
    }
}
