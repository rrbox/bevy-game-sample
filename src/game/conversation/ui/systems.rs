use crate::game::conversation::domain_models::Flow;
use crate::game::conversation::events::DisplayPassageEvent;
use crate::game::conversation::static_resources::Conversation;
use crate::game::conversation::ui::components::{
    ConversationPassageText, ConversationTellerText, ConversationUi,
};
use crate::game::game_flow::flow::CurrentStep;
use crate::game::shared::domain_models::conversation::start_conversation::StartConversationEvent;
use crate::game::states::GameState;
use bevy::prelude::*;

pub fn setup_conversation_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                visibility: Visibility::Hidden,
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(150.0),
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(0.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::FlexStart,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                background_color: Color::rgba(0.1, 0.1, 0.1, 0.8).into(),
                ..default()
            },
            ConversationUi::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 25.0,
                        color: Color::YELLOW,
                        ..default()
                    },
                ),
                ConversationTellerText,
            ));
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font: asset_server
                            .load("fonts/DotGothic16-Regular/DotGothic16-Regular.ttf"),
                        font_size: 50.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                ConversationPassageText,
            ));
        });
}

pub fn advance_conversation_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut conversation_ui_query: Query<(&mut ConversationUi, &mut Visibility)>,
    mut next_state: ResMut<NextState<GameState>>,
    conversation_res: Res<Conversation>,
    mut display_passage_event_writer: EventWriter<DisplayPassageEvent>,
    mut current_step: ResMut<CurrentStep>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let (mut conversation_ui, mut visibility) = conversation_ui_query.single_mut();

        if let Some((scenario_id, passage_id)) = conversation_ui.0.clone() {
            if let Some(scenario) = conversation_res.scenarios.get(&scenario_id) {
                if let Some(passage) = scenario.get_passage(&passage_id) {
                    match passage.flow.clone() {
                        Flow::Linear { next } => {
                            if let Some(next_passage) = scenario.get_passage(&next) {
                                display_passage_event_writer
                                    .send(DisplayPassageEvent(next_passage.clone()));
                                conversation_ui.0 = Some((scenario_id, next));
                            }
                        }
                        Flow::End { next_step } => {
                            next_state.set(GameState::Idle);
                            *visibility = Visibility::Hidden;
                            conversation_ui.0 = None;
                            current_step.0 = next_step;
                        }
                        Flow::Branch { .. } => {
                            // TODO: Implement branch logic
                        }
                    }
                }
            }
        }
    }
}

pub fn handle_start_conversation_event_system(
    mut conversation_event_reader: EventReader<StartConversationEvent>,
    mut conversation_ui_query: Query<(&mut ConversationUi, &mut Visibility)>,
    conversation_res: Res<Conversation>,
    mut display_passage_event_writer: EventWriter<DisplayPassageEvent>,
) {
    for event in conversation_event_reader.read() {
        let scenario_id = &event.0;
        if let Some(scenario) = conversation_res.scenarios.get(scenario_id) {
            let start_passage_id = scenario.start;
            if let Some(start_passage) = scenario.get_passage(&start_passage_id) {
                let (mut conversation_ui, mut visibility) = conversation_ui_query.single_mut();

                *visibility = Visibility::Visible;
                conversation_ui.0 = Some((scenario_id.clone(), start_passage_id));
                display_passage_event_writer.send(DisplayPassageEvent(start_passage.clone()));
            }
        }
    }
}

pub fn display_passage_system(
    mut display_passage_event_reader: EventReader<DisplayPassageEvent>,
    mut teller_text_query: Query<
        &mut Text,
        (
            With<ConversationTellerText>,
            Without<ConversationPassageText>,
        ),
    >,
    mut passage_text_query: Query<
        &mut Text,
        (
            With<ConversationPassageText>,
            Without<ConversationTellerText>,
        ),
    >,
) {
    for event in display_passage_event_reader.read() {
        let passage = &event.0;
        if let Ok(mut teller_text) = teller_text_query.get_single_mut() {
            teller_text.sections[0].value = passage.teller.clone();
        }
        if let Ok(mut passage_text) = passage_text_query.get_single_mut() {
            passage_text.sections[0].value = passage.text.clone();
        }
    }
}
