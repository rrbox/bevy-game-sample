use bevy::prelude::*;

use crate::game::databases::{
    battle::battle_key::BattleKey, conversation::conversation_key::ConversationKey,
    movie::movie_key::MovieKey, moving::moving_key::MovingKey,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum FlowStep {
    Conversation { conversation_key: ConversationKey },
    Moving { moving_key: MovingKey },
    Battle { battle_key: BattleKey },
    Movie { movie_key: MovieKey },
}

#[derive(Resource, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CurrentStep(u32);

impl From<u32> for CurrentStep {
    fn from(step: u32) -> Self {
        Self(step)
    }
}

#[derive(Resource)]
pub struct GameFlow {
    flow: HashMap<CurrentStep, FlowStep>,
}

impl GameFlow {
    pub fn new() -> Self {
        let mut flow = HashMap::new();
        flow.insert(
            0.into(),
            FlowStep::Moving {
                moving_key: "".into(),
            },
        );
        flow.insert(
            1.into(),
            FlowStep::Conversation {
                conversation_key: "".into(),
            },
        );
        Self { flow: flow }
    }
}
