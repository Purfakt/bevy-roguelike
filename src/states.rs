use bevy::prelude::*;

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq, SystemSet)]
pub enum MainState {
    #[default]
    LoadAssets,
    Game,
}

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq, SystemSet)]
pub enum GameState {
    #[default]
    None,
    PlayerInput,
    TurnUpdate,
}
