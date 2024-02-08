use bevy::prelude::*;

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq, SystemSet)]
pub enum MainState {
    #[default]
    LoadAssets,
    Game,
    AwaitingInput,
    PlayerTurn,
    EnemyTurn,
    GameOver,
    Victory,
    NextLevel,
}
