use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, systems::spawn_player)
            .add_systems(Update, systems::player_movement_system.in_set(systems::PlayerMovementSet));
    }
}