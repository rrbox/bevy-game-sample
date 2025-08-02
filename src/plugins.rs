use crate::game::player::PlayerPlugin;
use crate::game::ui::UiPlugin;
use crate::game::world::WorldPlugin;
use bevy::prelude::*;

pub struct GamePlugins;

impl Plugin for GamePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin)
            .add_plugins(UiPlugin)
            .add_plugins(WorldPlugin);
    }
}
