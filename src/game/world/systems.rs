// World systems
use bevy::prelude::*;
use rand::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_random_squares(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
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

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CameraFollowSet;

pub fn camera_follow_player_system(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    player_query: Query<&GlobalTransform, With<crate::game::player::components::Player>>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation().x;
    camera_transform.translation.y = player_transform.translation().y;
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
