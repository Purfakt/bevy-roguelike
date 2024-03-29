use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{
    actions::{
        models::{MeleeAttackAction, WalkAction},
        Action, ActionExecutedEvent,
    },
    board::components::Position,
    pieces::components::Piece,
};

use super::{
    components::PathAnimator, get_world_position, get_world_vec, GraphicsAssets, GraphicsWaitEvent, PIECE_SPEED,
    PIECE_Z, POSITION_TOLERANCE, TILE_SIZE,
};
#[derive(Component, Default)]
pub struct Actor(pub Option<Box<dyn Action>>);

pub fn path_animator_update(
    mut commands: Commands,
    mut query: Query<(Entity, &mut PathAnimator, &mut Transform)>,
    time: Res<Time>,
    mut ev_wait: EventWriter<super::GraphicsWaitEvent>,
) {
    for (entity, mut animator, mut transform) in query.iter_mut() {
        if animator.0.is_empty() {
            commands.entity(entity).remove::<PathAnimator>();
            continue;
        }
        ev_wait.send(super::GraphicsWaitEvent);
        let target = *animator.0.front().unwrap();
        let distance = (target - transform.translation).length();
        if distance > POSITION_TOLERANCE {
            transform.translation = transform.translation.lerp(target, PIECE_SPEED * time.delta_seconds());
        } else {
            transform.translation = target;
            animator.0.pop_front();
        }
    }
}

pub fn spawn_piece_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position, &Piece), Added<Piece>>,
    assets: Res<GraphicsAssets>,
) {
    for (entity, position, piece) in query.iter() {
        let sprite_idx = match piece.kind.as_str() {
            "Player" => 1,
            _ => 63,
        };
        let v = get_world_position(position, PIECE_Z);
        commands.entity(entity).insert(SpriteSheetBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..Default::default()
            },
            atlas: TextureAtlas {
                layout: assets.layout.clone(),
                index: sprite_idx,
            },
            texture: assets.texture.clone(),
            transform: Transform::from_translation(v),
            ..Default::default()
        });
    }
}

pub fn walk_animation(
    mut commands: Commands,
    mut event_action: EventReader<ActionExecutedEvent>,
    mut event_wait: EventWriter<GraphicsWaitEvent>,
) {
    for event in event_action.read() {
        let action = event.0.as_any();
        if let Some(action) = action.downcast_ref::<WalkAction>() {
            let target = get_world_vec(action.target_position, PIECE_Z);
            commands
                .entity(action.entity)
                .insert(PathAnimator(VecDeque::from([target])));
            event_wait.send(GraphicsWaitEvent);
        }
    }
}

pub fn melee_animation(
    mut commands: Commands,
    query: Query<&Position>,
    mut event_action: EventReader<ActionExecutedEvent>,
    mut event_wait: EventWriter<GraphicsWaitEvent>,
) {
    for event in event_action.read() {
        let action = event.0.as_any();
        if let Some(action) = action.downcast_ref::<MeleeAttackAction>() {
            let Ok(base_position) = query.get(action.attacker) else {
                continue;
            };
            let base = get_world_position(base_position, PIECE_Z);
            let target = 0.5 * (base + get_world_vec(action.target_position, PIECE_Z));
            commands
                .entity(action.attacker)
                .insert(PathAnimator(VecDeque::from([target, base])));
            event_wait.send(GraphicsWaitEvent);
        }
    }
}
