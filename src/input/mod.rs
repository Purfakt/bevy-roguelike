use bevy::prelude::*;

use crate::{
    board::components::Position,
    player::{DeckEvent, DeckEventKind, Player},
    states::GameState,
    vectors::Vector2Int,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_input.run_if(in_state(GameState::PlayerInput)));
    }
}

const DIR_KEY_MAPPING: [(KeyCode, Vector2Int); 4] = [
    (KeyCode::KeyW, Vector2Int::UP),
    (KeyCode::KeyS, Vector2Int::DOWN),
    (KeyCode::KeyA, Vector2Int::LEFT),
    (KeyCode::KeyD, Vector2Int::RIGHT),
];

fn player_input(
    keys: ResMut<ButtonInput<KeyCode>>,
    mut player_query: Query<&Position, With<Player>>,
    mut event_deck: EventWriter<DeckEvent>,
) {
    let Ok(position) = player_query.get_single_mut() else {
        return;
    };

    for (key, dir) in DIR_KEY_MAPPING {
        if !keys.just_pressed(key) {
            continue;
        }
        event_deck.send(DeckEvent(DeckEventKind::UseCard(Some(position.v + dir))));
    }
}
