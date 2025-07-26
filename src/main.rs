use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

const CAMERA_MOVE_SPEED: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, camera_movement_system)
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

fn camera_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    let mut camera_transform = camera_query.single_mut();

    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction += Vec3::new(-1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction += Vec3::new(0.0, -1.0, 0.0);
    }

    if direction != Vec3::ZERO {
        camera_transform.translation += direction.normalize() * CAMERA_MOVE_SPEED;
    }
}