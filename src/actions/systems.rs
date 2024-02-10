use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

use crate::{
    board::components::Position,
    pieces::components::{Actor, Walk},
    player::Player,
    vectors::ORTHO_DIRECTIONS,
};

use super::{models::WalkAction, ActionsCompleteEvent, ActorQueue, InvalidPlayerActionEvent, NextActorEvent};

pub fn populate_actor_queue(query: Query<Entity, (With<Actor>, Without<Player>)>, mut queue: ResMut<ActorQueue>) {
    queue.0.extend(query.iter());
}

pub fn process_action_queue(world: &mut World) {
    let Some(mut queue) = world.get_resource_mut::<ActorQueue>() else {
        return;
    };
    let Some(entity) = queue.0.pop_front() else {
        world.send_event(ActionsCompleteEvent);
        return;
    };
    let Some(mut actor) = world.get_mut::<Actor>(entity) else {
        return;
    };
    let Some(action) = actor.0.take() else { return };

    if !action.execute(world) && world.get::<Player>(entity).is_some() {
        world.send_event(InvalidPlayerActionEvent);
        return;
    }
    world.send_event(NextActorEvent);
}

pub fn plan_walk(mut query: Query<(&Position, &mut Actor), With<Walk>>, queue: Res<ActorQueue>) {
    let Some(entity) = queue.0.front() else { return };
    let Ok((position, mut actor)) = query.get_mut(*entity) else {
        return;
    };
    let mut rng = thread_rng();
    let dir = ORTHO_DIRECTIONS.choose(&mut rng).unwrap();
    actor.0 = Some(Box::new(WalkAction(*entity, position.0 + *dir)));
}
