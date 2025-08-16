// Common components
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct MovementSpeed {
    pub speed: f32,
}

#[derive(Component, Default)]
pub struct ConversationTrigger {
    pub size: Vec2,
    pub is_contact: bool,
}
