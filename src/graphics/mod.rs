use bevy::prelude::*;

use crate::board::components::Position;

mod assets;
mod pieces;
mod tiles;

pub const TILE_SIZE: f32 = 32.;
pub const TILE_Z: f32 = 0.;
pub const PIECE_Z: f32 = 10.;
pub const PIECE_SPEED: f32 = 10.;
pub const POSITION_TOLERANCE: f32 = 0.1;

#[derive(Resource)]
pub struct GraphicsAssets {
    pub sprite_texture: Handle<TextureAtlas>,
}

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, assets::load_assets).add_systems(
            Update,
            (
                tiles::spawn_tile_renderer,
                pieces::spawn_piece_renderer,
                pieces::update_piece_position,
            ),
        );
    }
}

fn get_world_position(position: &Position, z: f32) -> Vec3 {
    Vec3::new(TILE_SIZE * position.0.x as f32, TILE_SIZE * position.0.y as f32, z)
}
