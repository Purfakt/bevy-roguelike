use bevy::prelude::*;

use crate::{board::components::Position, states::MainState, vectors::Vector2Int};

use self::components::{Actor, Health, MeleeAttack, Occupier, Piece, Walk};

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
        Health { value: 3 },
        MeleeAttack { damage: 1 },
        Occupier,
        Piece {
            kind: "NPC".to_string(),
        },
        Position {
            v: Vector2Int { x: 2, y: 0 },
        },
        Walk,
    ));

    commands.spawn((
        Actor::default(),
        Health { value: 3 },
        MeleeAttack { damage: 1 },
        Occupier,
        Piece {
            kind: "NPC".to_string(),
        },
        Position {
            v: Vector2Int { x: 5, y: 5 },
        },
        Walk,
    ));
}
