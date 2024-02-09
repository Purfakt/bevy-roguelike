use bevy::prelude::*;

use crate::{
    board::{components::Position, CurrentBoard},
    vectors::Vector2Int,
};

use super::Action;

pub struct WalkAction(pub Entity, pub Vector2Int);

impl Action for WalkAction {
    fn execute(&self, world: &mut World) -> bool {
        let Some(board) = world.get_resource::<CurrentBoard>() else {
            return false;
        };
        if !board.tiles.contains_key(&self.1) {
            return false;
        };

        let Some(mut position) = world.get_mut::<Position>(self.0) else {
            return false;
        };
        position.0 = self.1;
        true
    }
}
