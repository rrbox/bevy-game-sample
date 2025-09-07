use bevy::prelude::*;
mod game_state;
mod states;
pub use states::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .init_state::<GameState>()
            .init_state::<PauseState>()
            .add_systems(
                Update,
                game_state::idle::advance_game_state
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameState::Idle)),
            );
    }
}
