use crate::game::states::{AppState, GameState};
use bevy::prelude::*;

// 起動時にAppStateをInGameに設定するシステム
pub fn force_ingame_state_system(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>
) {
    next_app_state.set(AppState::InGame);
    next_game_state.set(GameState::Moving);
}
