use bevy::prelude::*;
use std::{any::Any, collections::VecDeque};

use crate::states::GameState;

use self::systems::{plan_melee_attack, plan_walk, populate_actor_queue, process_action_queue};

pub mod models;
mod systems;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActorQueue>()
            .init_resource::<PendingActions>()
            .add_event::<TickEvent>()
            .add_event::<NextActorEvent>()
            .add_event::<ActionsCompleteEvent>()
            .add_event::<InvalidPlayerActionEvent>()
            .add_event::<ActionExecutedEvent>()
            .configure_sets(
                Update,
                (
                    ActionSet::Planning
                        .before(ActionSet::Late)
                        .run_if(on_event::<NextActorEvent>())
                        .run_if(in_state(GameState::TurnUpdate)),
                    ActionSet::Late.run_if(in_state(GameState::TurnUpdate)),
                ),
            )
            .add_systems(
                Update,
                (
                    process_action_queue.in_set(ActionSet::Late),
                    (plan_melee_attack, plan_walk).in_set(ActionSet::Planning),
                ),
            )
            .add_systems(OnExit(GameState::PlayerInput), populate_actor_queue);
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ActionSet {
    Planning,
    Late,
}

pub trait Action: Send + Sync {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()>;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Default, Resource)]
pub struct ActorQueue(pub VecDeque<Entity>);

#[derive(Default, Resource)]
pub struct PendingActions(pub Vec<Box<dyn Action>>);

#[derive(Event)]
pub struct TickEvent;

#[derive(Event)]
pub struct NextActorEvent;

#[derive(Event)]
pub struct ActionsCompleteEvent;

#[derive(Event)]
pub struct InvalidPlayerActionEvent;

#[derive(Event)]
pub struct ActionExecutedEvent(pub Box<dyn Action>);
