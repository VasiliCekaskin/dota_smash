pub struct GameState {
    pub current_player_count: u32,
}
impl Default for GameState {
    fn default() -> Self {
        Self {
            current_player_count: 0,
        }
    }
}
