use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CameraFollowSet;

const CAMERA_PLAYER_MAX_DISTANCE: f32 = 100.0;

pub fn camera_follow_player_system(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    player_query: Query<&GlobalTransform, With<crate::game::moving::player::components::Player>>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    let player_2d_pos = player_transform.translation().truncate();
    let camera_2d_pos = camera_transform.translation.truncate();
    let distance = player_2d_pos.distance(camera_2d_pos);

    if distance > CAMERA_PLAYER_MAX_DISTANCE {
        let direction = (player_2d_pos - camera_2d_pos).normalize();
        let overshoot = distance - CAMERA_PLAYER_MAX_DISTANCE;

        camera_transform.translation.x += direction.x * overshoot;
        camera_transform.translation.y += direction.y * overshoot;
    }
}
