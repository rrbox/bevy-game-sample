use crate::game::systems::CollisionPlugin;
use bevy::prelude::*;

pub mod game;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<game::ui::events::StartConversationEvent>()
            .add_plugins(game::player::PlayerPlugin)
            .add_plugins(game::ui::UiPlugin)
            .add_plugins(game::world::WorldPlugin)
            .add_plugins(CollisionPlugin)
            .configure_sets(
                Update,
                game::player::systems::PlayerMovementSet
                    .before(game::world::camera::systems::CameraFollowSet),
            );
    }
}
