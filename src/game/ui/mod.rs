use bevy::prelude::*;

pub mod pause;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(pause::PauseUiPlugin);
    }
}
