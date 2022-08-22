pub const FPS: f32 = 60.0;
pub const ROLLBACK_DEFAULT: &str = "rollback_default";

#[derive(PartialEq, Debug)]
pub enum GameStage {
    Menu,
    Init,
    SetupSocket,
    SetupSession,
    SpawnPlayers,
    Gameplay,
}

#[derive(Debug)]
pub struct GameState {
    pub socket_setup: bool,
    pub stage: GameStage,
}
