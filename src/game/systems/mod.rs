// Systems module
use crate::game::states::{AppState, PauseState};
use bevy::prelude::*;
pub mod collision;
pub mod combat;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            collision::check_conversation_trigger_system
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(PauseState::Running)),
        );
    }
}
