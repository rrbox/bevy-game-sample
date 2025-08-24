use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Title,
    EditPlayerName,
    InGame,
    Save,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Idle,
    Moving,
    Battle,
    Movie,
    Conversation,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum PauseState {
    #[default]
    Running,
    Pause,
}
