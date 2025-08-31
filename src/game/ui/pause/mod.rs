use crate::game::states::{AppState, GameState, PauseState};
use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct PauseUiPlugin;

impl Plugin for PauseUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            systems::toggle_pause_system
                .run_if(in_state(AppState::InGame).and_then(not(in_state(GameState::Idle)))),
        )
        .add_systems(OnEnter(PauseState::Pause), systems::spawn_pause_menu)
        .add_systems(OnEnter(PauseState::Running), systems::despawn_pause_menu);
    }
}
