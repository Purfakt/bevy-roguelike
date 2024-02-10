use std::collections::VecDeque;

use bevy::prelude::*;

use crate::actions::models::WalkAction;
use crate::actions::ActorQueue;
use crate::board::components::Position;
use crate::pieces::components::{Actor, PrioritizedAction};
use crate::player::Player;
use crate::states::GameState;
use crate::vectors::Vector2Int;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerInputReadyEvent>()
            .add_systems(Update, player_position.run_if(in_state(GameState::PlayerInput)));
    }
}

#[derive(Event)]
pub struct PlayerInputReadyEvent;

const DIR_KEY_MAPPING: [(KeyCode, Vector2Int); 4] = [
    (KeyCode::W, Vector2Int::UP),
    (KeyCode::S, Vector2Int::DOWN),
    (KeyCode::A, Vector2Int::LEFT),
    (KeyCode::D, Vector2Int::RIGHT),
];

fn player_position(
    keys: ResMut<Input<KeyCode>>,
    mut player_query: Query<(Entity, &Position, &mut Actor), With<Player>>,
    mut queue: ResMut<ActorQueue>,
    mut event_input: EventWriter<PlayerInputReadyEvent>,
) {
    let Ok((entity, position, mut actor)) = player_query.get_single_mut() else {
        return;
    };
    for (key, dir) in DIR_KEY_MAPPING {
        if !keys.just_pressed(key) {
            continue;
        }
        let action = WalkAction {
            entity,
            target_position: position.v + dir,
        };
        actor.potential_actions = vec![PrioritizedAction {
            action: Box::new(action),
            priority: 1,
        }];
        queue.0 = VecDeque::from([entity]);
        event_input.send(PlayerInputReadyEvent);
        info!("Input ready");
    }
}
