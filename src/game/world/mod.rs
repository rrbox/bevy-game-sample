use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (systems::spawn_random_squares, systems::spawn_camera))
            .add_systems(Update, systems::camera_follow_player_system.in_set(systems::CameraFollowSet));
    }
}