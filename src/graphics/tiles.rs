use bevy::prelude::*;

use crate::board::components::{Position, Tile};

use super::{get_world_position, GraphicsAssets, TILE_SIZE, TILE_Z};

pub fn spawn_tile_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position), Added<Tile>>,
    assets: Res<GraphicsAssets>,
) {
    for (entity, position) in query.iter() {
        let v = get_world_position(position, TILE_Z);
        commands.entity(entity).insert(SpriteSheetBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                color: Color::OLIVE,
                ..Default::default()
            },
            atlas: TextureAtlas {
                layout: assets.layout.clone(),
                index: 177,
            },
            texture: assets.texture.clone(),
            transform: Transform::from_translation(v),
            ..Default::default()
        });
    }
}
