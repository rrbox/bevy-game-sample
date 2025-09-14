use crate::game::conversation::domain_models::PassageID;
use crate::game::shared::domain_models::conversation::scenario_ids::ScenarioID;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct ConversationUi(pub Option<(ScenarioID, PassageID)>);

#[derive(Component)]
pub struct ConversationTellerText;

#[derive(Component)]
pub struct ConversationPassageText;
