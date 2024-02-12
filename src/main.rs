pub use bevy::prelude::*;
pub use bevy::utils::HashMap;

use crate::{
    camera::setup,
    globals::{SCREEN_HEIGHT, SCREEN_WIDTH},
    states::{GameState, MainState},
};

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
mod ui;
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
        .add_state::<MainState>()
        .add_state::<GameState>()
        .add_plugins((
            actions::ActionsPlugin,
            assets::AssetPlugin,
            board::BoardPlugin,
            graphics::GraphicsPlugin,
            input::InputPlugin,
            manager::ManagerPlugin,
            pieces::PiecesPlugin,
            player::PlayerPlugin,
            ui::UiPlugin,
        ))
        .add_systems(Startup, setup)
        .run()
}
