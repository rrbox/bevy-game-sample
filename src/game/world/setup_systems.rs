// World systems
use crate::game::moving::components::ConversationTrigger;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

const NUM_SQUARES: u32 = 1000;
const SQUARE_SIZE: f32 = 50.0;

pub fn spawn_random_squares(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let mut rng = rand::thread_rng();

    for _ in 0..NUM_SQUARES {
        let x = rng.gen_range(
            -window.width() / 2.0 + SQUARE_SIZE / 2.0
                ..(window.width() * 10.0) / 2.0 - SQUARE_SIZE / 2.0,
        );
        let y = rng.gen_range(
            -window.height() / 2.0 + SQUARE_SIZE / 2.0
                ..(window.height() * 10.0) / 2.0 - SQUARE_SIZE / 2.0,
        );

        let color = Color::rgb(134.0 / 255.0, 74.0 / 255.0, 43.0 / 255.0);

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
            ..default()
        });
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_conversation_trigger(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 1.0, 0.0), // Green square
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(150.0, 0.0, 0.0), // Position it to the right of the player's start
            ..default()
        },
        ConversationTrigger {
            size: Vec2::new(50.0, 50.0),
            ..default()
        },
    ));
}
