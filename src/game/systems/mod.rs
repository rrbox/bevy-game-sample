// Systems module
use bevy::prelude::*;
pub mod collision;
pub mod combat;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision::check_conversation_trigger_system);
    }
}
