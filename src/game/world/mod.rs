use crate::game::states::{AppState, PauseState};
use bevy::prelude::*;

pub mod camera;
pub mod components;
pub mod setup_systems;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            camera::systems::CameraFollowSet
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(PauseState::Running)), // <-- この行を追加
        )
        .add_systems(
            Startup,
            (
                setup_systems::spawn_random_squares,
                setup_systems::spawn_camera,
                setup_systems::spawn_conversation_trigger,
            ),
        )
        .add_systems(
            Update,
            camera::systems::camera_follow_player_system.in_set(camera::systems::CameraFollowSet),
        );
    }
}
