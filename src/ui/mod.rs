use std::collections::HashMap;

use bevy::prelude::*;

use crate::states::GameState;

use self::{
    assets::load_assets,
    deck::{button_click_animation, card_click, draw_deck},
};

mod assets;
mod deck;
mod helpers;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ReloadUiEvent>()
            .add_systems(Startup, load_assets)
            .add_systems(
                Update,
                (
                    button_click_animation,
                    draw_deck.run_if(on_event::<ReloadUiEvent>()),
                    card_click.in_set(GameState::PlayerInput),
                ),
            )
            .add_systems(OnEnter(GameState::PlayerInput), player_input_start);
    }
}

#[derive(Event)]
pub struct ReloadUiEvent;

fn player_input_start(mut ev_ui: EventWriter<ReloadUiEvent>) {
    ev_ui.send(ReloadUiEvent);
}

#[derive(Resource)]
pub struct UiAssets {
    pub font: Handle<Font>,
    pub textures: HashMap<&'static str, Handle<Image>>,
}
