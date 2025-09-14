use bevy::prelude::*;

use self::{
    battle::battle_database::BattleDataBase, movie::movie_database::MovieDataBase,
    moving::moving_database::MovingDataBase,
};

pub mod battle;
pub mod movie;
pub mod moving;

pub struct DatabasePlugin;

impl Plugin for DatabasePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BattleDataBase {})
            .insert_resource(MovieDataBase {})
            .insert_resource(MovingDataBase {});
    }
}
