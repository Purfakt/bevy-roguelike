use bevy::prelude::*;

use crate::{
    actions::{ActionsCompleteEvent, InvalidPlayerActionEvent, TickEvent},
    graphics::GraphicsWaitEvent,
    player::PlayerActionEvent,
    states::{GameState, MainState, TurnSet},
};

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            ((TurnSet::Logic, TurnSet::Animation, TurnSet::Tick)
                .chain()
                .in_set(GameState::TurnUpdate),),
        )
        .add_systems(OnEnter(MainState::Game), game_start)
        .add_systems(OnExit(MainState::Game), game_end)
        .add_systems(
            Update,
            (
                turn_update_start.run_if(on_event::<PlayerActionEvent>()),
                tick.run_if(in_state(GameState::TurnUpdate)),
                turn_update_end
                    .run_if(on_event::<ActionsCompleteEvent>())
                    .run_if(in_state(GameState::TurnUpdate)),
                turn_update_cancel.run_if(on_event::<InvalidPlayerActionEvent>()),
            )
                .chain(),
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
