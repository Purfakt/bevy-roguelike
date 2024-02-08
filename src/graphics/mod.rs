use bevy::prelude::*;

mod assets;
mod tiles;

pub const TILE_SIZE: f32 = 32.;

#[derive(Resource)]
pub struct GraphicsAssets {
    pub sprite_texture: Handle<TextureAtlas>,
}

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, assets::load_assets)
            .add_systems(Update, tiles::spawn_tile_renderer);
    }
}
