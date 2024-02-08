use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::vectors::Vector2Int;

#[derive(Component)]
pub struct Position(pub Vector2Int);

#[derive(Component)]
pub struct Tile;

#[derive(Default, Resource)]
pub struct BoardRes {
    pub tiles: HashMap<Vector2Int, Entity>,
}
