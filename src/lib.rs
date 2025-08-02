use bevy::prelude::*;

pub mod events;
pub mod game;
pub mod plugins;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(game::player::PlayerPlugin)
            .add_plugins(game::ui::UiPlugin)
            .add_plugins(game::world::WorldPlugin)
            .configure_sets(
                Update,
                game::player::systems::PlayerMovementSet
                    .before(game::world::systems::CameraFollowSet),
            );
    }
}
