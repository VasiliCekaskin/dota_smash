#[derive(PartialEq, Debug)]
pub enum GameStage {
    Init,
    SetupSocket,
    SetupSession,
    SpawnPlayers,
    Gameplay,
}

#[derive(Debug)]
pub struct GameState {
    pub stage: GameStage,
}
