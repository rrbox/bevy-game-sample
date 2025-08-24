use crate::game::states::{AppState, PauseState};
use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod systems;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::setup_conversation_ui)
            // 会話の開始/終了処理はポーズ中は停止
            .add_systems(
                Update,
                (
                    systems::close_conversation_ui_system,
                    systems::handle_start_conversation_event_system,
                )
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(PauseState::Running)),
            )
            // ポーズ切り替え自体は、ポーズ中でも動作させる
            .add_systems(
                Update,
                systems::toggle_pause_system.run_if(in_state(AppState::InGame)),
            )
            // ポーズ状態の遷移時にUIを制御
            .add_systems(OnEnter(PauseState::Pause), systems::spawn_pause_menu)
            .add_systems(OnEnter(PauseState::Running), systems::despawn_pause_menu);
    }
}
