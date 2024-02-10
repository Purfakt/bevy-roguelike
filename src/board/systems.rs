use bevy::prelude::*;
use bevy::utils::HashMap;

use super::components::Position;
use super::components::Tile;
use super::CurrentBoard;
use crate::vectors::Vector2Int;

pub fn spawn_map(mut commands: Commands, mut current: ResMut<CurrentBoard>) {
    current.tiles = HashMap::new();
    for x in 0..8 {
        for y in 0..8 {
            let v = Vector2Int::new(x, y);
            let tile = commands.spawn((Position { v }, Tile)).id();
            current.tiles.insert(v, tile);
        }
    }
}
