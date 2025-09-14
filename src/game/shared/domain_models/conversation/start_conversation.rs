// Centralized definition for game events
use bevy::prelude::*;

use crate::game::shared::domain_models::conversation::scenario_ids::ScenarioID;

#[derive(Event)]
pub struct StartConversationEvent(pub ScenarioID);
