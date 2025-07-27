use bevy::{prelude::*, window::PrimaryWindow, ui::*};
use rand::prelude::*;

const PLAYER_MOVE_SPEED: f32 = 10.0;

// 新しいコンポーネントを定義
#[derive(Component)]
struct ConversationUi;

// Playerを識別するためのコンポーネント
#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, setup_conversation_ui))
        .add_systems(Update, (player_and_camera_movement_system, toggle_conversation_ui)) // 新しいシステムを追加
        .run();
}

fn setup(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // カメラを追加
    commands.spawn(Camera2dBundle::default());

    // Player Entity を追加
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
    ));

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

fn setup_conversation_ui(mut commands: Commands) {
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(150.0), // Height of the conversation UI
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            background_color: Color::rgba(0.1, 0.1, 0.1, 0.8).into(), // Dark semi-transparent background
            ..default()
        },
        ConversationUi, // Add the new component
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Hello, adventurer! Welcome to the dungeon.",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}



fn player_and_camera_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>, // Without<Player> を追加
) {
    let mut player_transform = player_query.single_mut();
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
        player_transform.translation += direction.normalize() * PLAYER_MOVE_SPEED;
    }

    // カメラをプレイヤーに追従させる
    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn toggle_conversation_ui(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut conversation_ui_query: Query<&mut Visibility, With<ConversationUi>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let mut visibility = conversation_ui_query.single_mut();
        *visibility = match *visibility {
            Visibility::Visible => Visibility::Hidden,
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Visible, // Other states like `Inherited` or `Displayed` will become `Visible`
        };
    }
}
