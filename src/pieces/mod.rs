use bevy::prelude::*;

use crate::{board::components::Position, states::MainState, vectors::Vector2Int};

use self::components::{Actor, MeleeAttack, Occupier, Piece, Walk};

pub mod components;

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Game), spawn_npc);
    }
}

fn spawn_npc(mut commands: Commands) {
    commands.spawn((
        Actor::default(),
        Piece {
            kind: "NPC".to_string(),
        },
        Position {
            v: Vector2Int { x: 5, y: 3 },
        },
        Occupier,
        Walk,
        MeleeAttack { damage: 1 },
    ));

    commands.spawn((
        Actor::default(),
        Piece {
            kind: "NPC".to_string(),
        },
        Position {
            v: Vector2Int { x: 5, y: 5 },
        },
        Occupier,
        Walk,
        MeleeAttack { damage: 1 },
    ));
}
