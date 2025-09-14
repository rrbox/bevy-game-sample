use bevy::prelude::Event;

use super::domain_models::Passage;

#[derive(Event)]
pub struct DisplayPassageEvent(pub Passage);
