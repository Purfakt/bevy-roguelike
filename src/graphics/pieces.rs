use bevy::prelude::*;

use crate::{actions::Action, board::components::Position, pieces::components::Piece};

use super::{
    get_world_position, GraphicsAssets, GraphicsWaitEvent, PIECE_SPEED, PIECE_Z, POSITION_TOLERANCE, TILE_SIZE,
};
#[derive(Component, Default)]
pub struct Actor(pub Option<Box<dyn Action>>);

pub fn update_piece_position(
    mut query: Query<(&Position, &mut Transform), With<Piece>>,
    time: Res<Time>,
    mut event_wait: EventWriter<GraphicsWaitEvent>,
) {
    let mut animating = false;
    for (position, mut transform) in query.iter_mut() {
        let target = get_world_position(position, PIECE_Z);
        let d = (target - transform.translation).length();
        if d > POSITION_TOLERANCE {
            transform.translation = transform.translation.lerp(target, PIECE_SPEED * time.delta_seconds());
            animating = true;
        } else {
            transform.translation = target;
        }
    }

    if animating {
        event_wait.send(GraphicsWaitEvent);
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
        let mut sprite = TextureAtlasSprite::new(sprite_idx);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
        sprite.color = Color::WHITE;
        let v = get_world_position(position, PIECE_Z);
        commands.entity(entity).insert(SpriteSheetBundle {
            sprite,
            texture_atlas: assets.sprite_texture.clone(),
            transform: Transform::from_translation(v),
            ..Default::default()
        });
    }
}
