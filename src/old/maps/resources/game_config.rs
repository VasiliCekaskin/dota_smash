use bevy::prelude::Vec2;

const PLAYER_MAX_COUNT: u32 = 2;
const PLAYER_MOVEMENT_TRANSLATION_VECTOR: Vec2 = Vec2 { x: 1.0, y: 1.0 };
const PLAYER_MAX_MOVEMENT_SPEED: f32 = 500.0;
const PLAYER_JUMP_SPEED: f32 = 800.0;
const PLAYER_ACCELERATION: Vec2 = Vec2 {
    x: PLAYER_MAX_MOVEMENT_SPEED,
    y: PLAYER_JUMP_SPEED,
};

pub struct GameConfig {
    pub player_max_count: u32,
    pub player_movement_translation_vector: Vec2,
    pub player_max_movement_speed: f32,
    pub player_acceleration: Vec2,
}
impl Default for GameConfig {
    fn default() -> Self {
        Self {
            player_max_count: PLAYER_MAX_COUNT,
            player_movement_translation_vector:
                PLAYER_MOVEMENT_TRANSLATION_VECTOR,
            player_max_movement_speed: PLAYER_MAX_MOVEMENT_SPEED,
            player_acceleration: PLAYER_ACCELERATION,
        }
    }
}
