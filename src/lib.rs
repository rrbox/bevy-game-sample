use crate::game::states::{AppState, GameState, PauseState};
use crate::game::systems::CollisionPlugin;
use bevy::prelude::*;

pub mod game;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .init_state::<GameState>()
            .init_state::<PauseState>()
            .add_plugins(game::databases::DatabasePlugin)
            .add_plugins(game::game_flow::GameFlowPlugin)
            .add_event::<game::ui::conversation::events::StartConversationEvent>()
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
