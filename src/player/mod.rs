use bevy::prelude::*;

use crate::board::components::Position;
use crate::pieces::components::{Actor, Piece};
use crate::states::MainState;
use crate::vectors::Vector2Int;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Game), spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Actor::default(),
        Piece {
            kind: "Player".to_string(),
        },
        Position(Vector2Int { x: 0, y: 0 }),
    ));
}
