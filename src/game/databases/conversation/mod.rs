use bevy::prelude::*;

use crate::game::databases::conversation::conversation_database::ConversationDataBase;

pub mod conversation_database;
pub mod conversation_key;

pub struct ConversationPlugin;

impl Plugin for ConversationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConversationDataBase {});
    }
}
