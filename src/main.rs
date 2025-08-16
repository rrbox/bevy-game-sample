use bevy::prelude::*;
use bevy_game_sample::GamePlugin; // Assuming your crate name is sample_3

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}
