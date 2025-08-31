use self::flow::{CurrentStep, GameFlow, StepID};
use bevy::prelude::*;

pub mod flow;

pub struct GameFlowPlugin;

impl Plugin for GameFlowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameFlow::new())
            .insert_resource(CurrentStep(StepID::from(0)));
    }
}
