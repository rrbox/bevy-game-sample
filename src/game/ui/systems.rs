// UI systems
use crate::game::ui::components::ConversationUi;
use crate::game::ui::events::StartConversationEvent;
use bevy::prelude::*;

pub fn setup_conversation_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                visibility: Visibility::Hidden,
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

pub fn close_conversation_ui_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut conversation_ui_query: Query<&mut Visibility, With<ConversationUi>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let mut visibility = conversation_ui_query.single_mut();
        if *visibility == Visibility::Visible {
            *visibility = Visibility::Hidden;
        }
    }
}

pub fn handle_start_conversation_event_system(
    mut conversation_event_reader: EventReader<StartConversationEvent>,
    mut conversation_ui_query: Query<&mut Visibility, With<ConversationUi>>,
) {
    for _event in conversation_event_reader.read() {
        let mut visibility = conversation_ui_query.single_mut();
        *visibility = Visibility::Visible;
    }
}
