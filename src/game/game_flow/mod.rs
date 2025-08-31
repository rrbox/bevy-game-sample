use bevy::prelude::*;
// Use `self` to refer to the module's own submodules
use self::flow::{CurrentStep, GameFlow};

pub mod flow;

pub struct GameFlowPlugin;

impl Plugin for GameFlowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameFlow::new())
            .insert_resource(CurrentStep::from(0));
    }
}
