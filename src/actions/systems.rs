use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    board::{components::Position, CurrentBoard},
    pieces::components::{Actor, MeleeAttack, Occupier, PrioritizedAction, Walk},
    player::Player,
    vectors::{find_path, ORTHO_DIRECTIONS},
};

use super::{
    models::{MeleeAttackAction, WalkAction},
    ActionsCompleteEvent, ActorQueue, InvalidPlayerActionEvent, NextActorEvent, PendingActions,
};

pub const MOVE_SCORE: i32 = 50;
pub const ATTACK_SCORE: i32 = 100;

pub fn populate_actor_queue(query: Query<Entity, (With<Actor>, Without<Player>)>, mut queue: ResMut<ActorQueue>) {
    queue.0.extend(query.iter());
}

pub fn process_action_queue(world: &mut World) {
    if process_pending_actions(world) {
        return;
    }

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
    let mut possible_actions = actor.potential_actions.drain(..).collect::<Vec<_>>();
    possible_actions.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap());

    let mut success = false;

    for action in possible_actions {
        if let Ok(result) = action.action.execute(world) {
            if let Some(mut pending) = world.get_resource_mut::<PendingActions>() {
                pending.0 = result;
            }
            success = true;
            break;
        }
    }

    if !success && world.get::<Player>(entity).is_some() {
        world.send_event(InvalidPlayerActionEvent);
        return;
    }

    world.send_event(NextActorEvent);
}

fn process_pending_actions(world: &mut World) -> bool {
    let pending = match world.get_resource_mut::<PendingActions>() {
        Some(mut res) => res.0.drain(..).collect::<Vec<_>>(),
        None => return false,
    };
    let mut new_actions = Vec::new();
    let mut success = false;
    for action in pending {
        if let Ok(result) = action.execute(world) {
            new_actions.extend(result);
            success = true;
        }
    }

    let mut res = world.get_resource_mut::<PendingActions>().unwrap();
    res.0 = new_actions;
    success
}

pub fn plan_walk(
    mut query: Query<(&Position, &mut Actor), With<Walk>>,
    queue: Res<ActorQueue>,
    player_query: Query<&Position, With<Player>>,
    occupier_query: Query<&Position, With<Occupier>>,
    board: Res<CurrentBoard>,
) {
    let Some(entity) = queue.0.front() else { return };
    let Ok((position, mut actor)) = query.get_mut(*entity) else {
        return;
    };
    let Ok(player_position) = player_query.get_single() else {
        return;
    };
    let positions = ORTHO_DIRECTIONS.iter().map(|dir| position.v + *dir).collect::<Vec<_>>();
    let path_to_player = find_path(
        position.v,
        player_position.v,
        &board.tiles.keys().cloned().collect(),
        &occupier_query.iter().map(|p| p.v).collect(),
    );

    let mut rng = thread_rng();
    let actions = positions.iter().map(|v| {
        let mut d = rng.gen_range(-10..0);
        if let Some(path) = &path_to_player {
            if path.contains(v) {
                d = 5;
            }
        }

        PrioritizedAction {
            action: Box::new(WalkAction {
                entity: *entity,
                target_position: *v,
            }),
            priority: MOVE_SCORE + d,
        }
    });

    actor.potential_actions.extend(actions);
}

pub fn plan_melee_attack(
    mut query: Query<(&mut Actor, &MeleeAttack)>,
    player_query: Query<&Position, With<Player>>,
    queue: Res<ActorQueue>,
) {
    let Some(entity) = queue.0.front() else {
        return;
    };
    let Ok((mut actor, melee)) = query.get_mut(*entity) else {
        return;
    };
    let Ok(player_position) = player_query.get_single() else {
        return;
    };
    let action = Box::new(MeleeAttackAction {
        attacker: *entity,
        target_position: player_position.v,
        damage: melee.damage,
    });

    actor.potential_actions.push(PrioritizedAction {
        action,
        priority: ATTACK_SCORE + melee.damage as i32,
    });
}
