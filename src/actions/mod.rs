use bevy::prelude::*;
use std::collections::VecDeque;

use crate::states::GameState;

use self::systems::{plan_walk, populate_actor_queue, process_action_queue};

pub mod models;
mod systems;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActorQueue>()
            .add_event::<TickEvent>()
            .add_event::<NextActorEvent>()
            .add_event::<ActionsCompleteEvent>()
            .add_event::<InvalidPlayerActionEvent>()
            .add_systems(Update, process_action_queue.run_if(on_event::<TickEvent>()))
            .add_systems(
                Update,
                plan_walk
                    .run_if(on_event::<NextActorEvent>())
                    .run_if(in_state(GameState::TurnUpdate)),
            )
            .add_systems(OnExit(GameState::PlayerInput), populate_actor_queue);
    }
}

pub trait Action: Send + Sync {
    fn execute(&self, world: &mut World) -> bool;
}

#[derive(Default, Resource)]
pub struct ActorQueue(pub VecDeque<Entity>);

#[derive(Event)]
pub struct TickEvent;

#[derive(Event)]
pub struct NextActorEvent;

#[derive(Event)]
pub struct ActionsCompleteEvent;

#[derive(Event)]
pub struct InvalidPlayerActionEvent;
