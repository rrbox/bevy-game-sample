// Collision systems
use crate::game::common::components::ConversationTrigger;
use crate::game::game_flow::flow::CurrentStep;
use crate::game::moving::player::components::Player;
use crate::game::states::GameState;
use bevy::math::bounding::Aabb2d;
use bevy::math::bounding::IntersectsVolume;
use bevy::prelude::*;

pub fn check_conversation_trigger_system(
    player_query: Query<(&Transform, &Sprite), With<Player>>,
    mut trigger_query: Query<(&Transform, &mut ConversationTrigger)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut current_step: ResMut<CurrentStep>,
) {
    if let Ok((player_transform, player_sprite)) = player_query.get_single() {
        let player_size = player_sprite.custom_size.unwrap_or(Vec2::ONE);
        let player_aabb = Aabb2d::new(player_transform.translation.truncate(), player_size / 2.0);

        for (trigger_transform, mut trigger) in trigger_query.iter_mut() {
            let trigger_aabb =
                Aabb2d::new(trigger_transform.translation.truncate(), trigger.size / 2.0);

            if player_aabb.intersects(&trigger_aabb) {
                if !trigger.is_contact {
                    next_state.set(GameState::Idle);
                    current_step.0 = 1.into();
                    trigger.is_contact = true;
                }
            } else {
                trigger.is_contact = false;
            }
        }
    }
}
