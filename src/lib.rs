use crate::game::states::StatePlugin;
use crate::game::systems::CollisionPlugin;
use bevy::prelude::*;

pub mod game;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StatePlugin)
            .add_plugins(game::databases::DatabasePlugin)
            .add_plugins(game::game_flow::GameFlowPlugin)
            .add_event::<game::ui::conversation::events::StartConversationEvent>()
            .add_plugins(game::moving::player::PlayerPlugin)
            .add_plugins(game::ui::UiPlugin)
            .add_plugins(game::world::WorldPlugin)
            .add_plugins(CollisionPlugin)
            .configure_sets(
                Update,
                game::moving::player::systems::PlayerMovementSet
                    .before(game::world::camera::systems::CameraFollowSet),
            );
    }
}
