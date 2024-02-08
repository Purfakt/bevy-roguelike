use bevy::asset::LoadState;
use bevy::prelude::*;

use crate::states::MainState;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetList>()
            .add_systems(Update, check_asset_loading.in_set(MainState::LoadAssets));
    }
}

#[derive(Default, Resource)]
pub struct AssetList(pub Vec<UntypedHandle>);

pub fn check_asset_loading(
    asset_server: Res<AssetServer>,
    asset_list: Res<AssetList>,
    mut next_state: ResMut<NextState<MainState>>,
) {
    if let Some(id) = asset_list.0.iter().map(|a| a.id()).next() {
        match asset_server.get_load_state(id) {
            Some(LoadState::Loaded) => {
                next_state.set(MainState::Game);
            }
            Some(LoadState::Failed) => {
                error!("asset loading error");
            }
            Some(_) => todo!(),
            None => todo!(),
        };
    }
}
