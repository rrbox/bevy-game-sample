use bevy::prelude::*;

pub mod conversation;
pub mod pause;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((conversation::ConversationUiPlugin, pause::PauseUiPlugin));
    }
}
