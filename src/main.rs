use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // カメラを追加
    commands.spawn(Camera2dBundle::default());

    let window = window_query.get_single().unwrap();
    let mut rng = rand::thread_rng();

    const NUM_SQUARES: u32 = 10;
    const SQUARE_SIZE: f32 = 50.0;

    for _ in 0..NUM_SQUARES {
        let x = rng.gen_range(-window.width() / 2.0 + SQUARE_SIZE / 2.0..window.width() / 2.0 - SQUARE_SIZE / 2.0);
        let y = rng.gen_range(-window.height() / 2.0 + SQUARE_SIZE / 2.0..window.height() / 2.0 - SQUARE_SIZE / 2.0);

        let color = Color::rgb(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0));

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