use bevy::prelude::*;

use super::GraphicsAssets;

const ATLAS_PATH: &str = "ascii.png";

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlasses: ResMut<Assets<TextureAtlasLayout>>,
    mut asset_list: ResMut<crate::assets::AssetList>,
) {
    let texture: Handle<Image> = asset_server.load(ATLAS_PATH);
    asset_list.0.push(texture.clone().untyped());
    let atlas_layout = TextureAtlasLayout::from_grid(Vec2::splat(10.), 16, 16, None, None);
    let layout = texture_atlasses.add(atlas_layout);
    commands.insert_resource(GraphicsAssets { texture, layout });
}
