use crate::game::states::{AppState, GameState, PauseState};
use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            systems::PlayerMovementSet
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameState::Moving))
                .run_if(in_state(PauseState::Running)),
        )
        .add_systems(Startup, systems::spawn_player)
        .add_systems(
            Update,
            (
                systems::player_movement_system,
                systems::player_movement_speed_system,
            )
                .in_set(systems::PlayerMovementSet),
        );
    }
}
