use bevy::app::Plugin;

use crate::game::conversation::ui::ConversationUiPlugin;

pub mod ui;

pub struct ConversationPlugin;

impl Plugin for ConversationPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(ConversationUiPlugin);
    }
}
