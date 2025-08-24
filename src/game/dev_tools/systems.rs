use crate::game::states::AppState;
use bevy::prelude::*;

// 起動時にAppStateをInGameに設定するシステム
pub fn force_ingame_state_system(mut next_app_state: ResMut<NextState<AppState>>) {
    next_app_state.set(AppState::InGame);
}
