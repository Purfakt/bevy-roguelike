use bevy::prelude::*;

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq, SystemSet)]
pub enum MainState {
    #[default]
    LoadAssets,
    Game,
    _AwaitingInput,
    _PlayerTurn,
    _EnemyTurn,
    _GameOver,
    _Victory,
    _NextLevel,
}

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq, SystemSet)]
pub enum GameState {
    #[default]
    None,
    PlayerInput,
    TurnUpdate,
}
