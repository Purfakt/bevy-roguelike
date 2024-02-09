pub use bevy::prelude::*;
pub use bevy::utils::HashMap;

use crate::camera::setup;
use crate::globals::{SCREEN_HEIGHT, SCREEN_WIDTH};

mod actions;
mod assets;
mod board;
mod camera;
mod globals;
mod graphics;
mod input;
mod manager;
mod pieces;
mod player;
mod states;
mod vectors;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(Msaa::Off)
        .add_state::<states::MainState>()
        .add_state::<states::GameState>()
        .add_plugins((
            actions::ActionsPlugin,
            assets::AssetPlugin,
            board::BoardPlugin,
            graphics::GraphicsPlugin,
            input::InputPlugin,
            manager::ManagerPlugin,
            player::PlayerPlugin,
        ))
        .add_systems(Startup, setup)
        .run()
}
