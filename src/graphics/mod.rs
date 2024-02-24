use bevy::prelude::*;

use crate::{board::components::Position, states::TurnSet, vectors::Vector2Int};

mod assets;
mod components;
mod pieces;
mod tiles;

pub const TILE_SIZE: f32 = 32.;
pub const TILE_Z: f32 = 0.;
pub const PIECE_Z: f32 = 10.;
pub const PIECE_SPEED: f32 = 10.;
pub const POSITION_TOLERANCE: f32 = 0.1;

#[derive(Resource)]
pub struct GraphicsAssets {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GraphicsWaitEvent>()
            .add_systems(Startup, assets::load_assets)
            .add_systems(
                Update,
                (
                    tiles::spawn_tile_renderer,
                    pieces::spawn_piece_renderer,
                    (
                        pieces::path_animator_update,
                        pieces::walk_animation,
                        pieces::melee_animation,
                    )
                        .in_set(TurnSet::Animation),
                ),
            );
    }
}

#[derive(Event)]
pub struct GraphicsWaitEvent;

fn get_world_vec(v: Vector2Int, z: f32) -> Vec3 {
    Vec3::new(TILE_SIZE * v.x as f32, TILE_SIZE * v.y as f32, z)
}

fn get_world_position(position: &Position, z: f32) -> Vec3 {
    get_world_vec(position.v, z)
}
