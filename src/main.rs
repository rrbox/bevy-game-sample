use bevy::prelude::*;
use bevy_game_sample::GamePlugin;
use bevy_game_sample::game::dev_tools::DevToolsPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(GamePlugin);

    // デバッグビルドのとき（cargo run など）だけ DevToolsPlugin を追加
    #[cfg(debug_assertions)]
    app.add_plugins(DevToolsPlugin);

    app.run();
}
