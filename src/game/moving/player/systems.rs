// Player systems
use crate::game::moving::components::MovementSpeed;
use crate::game::moving::player::components::Player;
use bevy::prelude::*;

pub const PLAYER_MOVE_SPEED: f32 = 8.0;
pub const PLAYER_SPRINT_MOVE_SPEED: f32 = 15.0;
pub const PLAYER_SLOW_MOVE_SPEED: f32 = 4.0;
pub const MAX_FRAME_PER_SECONDS: f32 = 60.0;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PlayerMovementSet;

pub fn player_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &MovementSpeed), With<Player>>,
) {
    let (mut player_transform, player_speed) = player_query.single_mut();
    let mut direction = Vec3::ZERO;

    if keyboard_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        direction += Vec3::new(-1.0, 0.0, 0.0);
    }
    if keyboard_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }
    if keyboard_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        direction += Vec3::new(0.0, -1.0, 0.0);
    }

    if direction != Vec3::ZERO {
        player_transform.translation += direction.normalize() * player_speed.speed;
    }
}

pub fn player_movement_speed_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut MovementSpeed, With<Player>>,
    time: Res<Time>,
) {
    let mut player_speed = player_query.single_mut();
    let delta_second = time.delta_seconds();
    let movement_speed = if keyboard_input.pressed(KeyCode::ShiftLeft) {
        PLAYER_SPRINT_MOVE_SPEED
    } else if keyboard_input.pressed(KeyCode::ControlLeft) {
        PLAYER_SLOW_MOVE_SPEED
    } else {
        PLAYER_MOVE_SPEED
    };
    player_speed.speed = delta_second * movement_speed * MAX_FRAME_PER_SECONDS;
}

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0), // 赤色の四角形
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Player,
        MovementSpeed { ..default() },
    ));
}
