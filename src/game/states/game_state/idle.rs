use bevy::prelude::*;

use crate::game::{
    databases::{
        battle::battle_database::BattleDataBase, movie::movie_database::MovieDataBase,
        moving::moving_database::MovingDataBase,
    },
    game_flow::flow::{CurrentStep, GameFlow},
    shared::domain_models::conversation::start_conversation::StartConversationEvent,
    states::GameState,
};

pub fn advance_game_state(
    flow: Res<GameFlow>,
    current_step: Res<CurrentStep>,
    mut conversation_event: EventWriter<StartConversationEvent>,
    moving_db: Res<MovingDataBase>,
    battle_db: Res<BattleDataBase>,
    movie_db: Res<MovieDataBase>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // flow resource から current flow step を取り出す
    // current flow step をパターンマッチングする
    // 分岐先で next_state を更新する
    if let Some(current_flow_step) = flow.get_step(&current_step.0) {
        match current_flow_step {
            crate::game::game_flow::flow::FlowStep::Conversation { scenario_id } => {
                next_state.set(GameState::Conversation);
                conversation_event.send(StartConversationEvent(scenario_id.clone()));
            }
            crate::game::game_flow::flow::FlowStep::Moving { moving_key } => {
                next_state.set(GameState::Moving);
            }
            crate::game::game_flow::flow::FlowStep::Battle { battle_key } => {
                // TODO: transition to Battle state
            }
            crate::game::game_flow::flow::FlowStep::Movie { movie_key } => {
                // TODO: transition to Movie state
            }
        }
    }
}
