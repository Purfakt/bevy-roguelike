use bevy::prelude::*;

use crate::{
    actions::{ActionsCompleteEvent, InvalidPlayerActionEvent, TickEvent},
    graphics::GraphicsWaitEvent,
    input::PlayerInputReadyEvent,
    states::{GameState, MainState},
};

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Game), game_start)
            .add_systems(OnExit(MainState::Game), game_end)
            .add_systems(
                Update,
                (
                    turn_update_start.run_if(on_event::<PlayerInputReadyEvent>()),
                    turn_update_end.run_if(on_event::<ActionsCompleteEvent>()),
                    turn_update_cancel.run_if(on_event::<InvalidPlayerActionEvent>()),
                    tick.in_set(GameState::TurnUpdate),
                ),
            );
    }
}

fn game_start(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::PlayerInput);
}

fn game_end(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::None);
}

fn turn_update_start(mut next_state: ResMut<NextState<GameState>>, mut event_tick: EventWriter<TickEvent>) {
    next_state.set(GameState::TurnUpdate);
    event_tick.send(TickEvent);
}

fn tick(mut event_wait: EventReader<GraphicsWaitEvent>, mut event_tick: EventWriter<TickEvent>) {
    if event_wait.read().len() == 0 {
        event_tick.send(TickEvent);
    }
}

fn turn_update_end(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::PlayerInput);
}

fn turn_update_cancel(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::PlayerInput);
}
