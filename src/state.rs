use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
    Paused,
    GameOver,
}

#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PhaseState {
    #[default]
    Idle,
    Matching,
    Cascading,
    Combating,
}
