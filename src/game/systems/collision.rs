// Collision systems
use crate::game::common::components::ConversationTrigger;
use crate::game::player::components::Player;
use crate::game::ui::conversation::events::StartConversationEvent;
use bevy::math::bounding::Aabb2d;
use bevy::math::bounding::IntersectsVolume;
use bevy::prelude::*;

pub fn check_conversation_trigger_system(
    player_query: Query<(&Transform, &Sprite), With<Player>>,
    mut trigger_query: Query<(&Transform, &mut ConversationTrigger)>,
    mut conversation_event_writer: EventWriter<StartConversationEvent>,
) {
    if let Ok((player_transform, player_sprite)) = player_query.get_single() {
        let player_size = player_sprite.custom_size.unwrap_or(Vec2::ONE);
        let player_aabb = Aabb2d::new(player_transform.translation.truncate(), player_size / 2.0);

        for (trigger_transform, mut trigger) in trigger_query.iter_mut() {
            let trigger_aabb =
                Aabb2d::new(trigger_transform.translation.truncate(), trigger.size / 2.0);

            if player_aabb.intersects(&trigger_aabb) {
                if !trigger.is_contact {
                    conversation_event_writer.send(StartConversationEvent);
                    trigger.is_contact = true;
                }
            } else {
                trigger.is_contact = false;
            }
        }
    }
}
