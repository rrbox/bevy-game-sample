use bevy::prelude::*;

use crate::game::{
    databases::{
        battle::battle_key::BattleKey, movie::movie_key::MovieKey, moving::moving_key::MovingKey,
    },
    shared::domain_models::conversation::scenario_ids::{ScenarioID, ScenarioIDs},
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum FlowStep {
    Conversation { scenario_id: ScenarioID },
    Moving { moving_key: MovingKey },
    Battle { battle_key: BattleKey },
    Movie { movie_key: MovieKey },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StepID(pub u32);

impl From<u32> for StepID {
    fn from(id: u32) -> Self {
        Self(id)
    }
}

#[derive(Resource)]
pub struct CurrentStep(pub StepID);

#[derive(Resource)]
pub struct GameFlow {
    flow: HashMap<StepID, FlowStep>,
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
                scenario_id: ScenarioIDs::start(),
            },
        );
        Self { flow: flow }
    }

    pub fn get_step(&self, step_id: &StepID) -> Option<&FlowStep> {
        self.flow.get(step_id)
    }
}
