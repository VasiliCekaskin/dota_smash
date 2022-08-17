use bevy::{
    input::keyboard::KeyboardInput,
    prelude::{
        AssetServer, Commands, Component, EventReader, EventWriter, KeyCode,
        Plugin, Query, Res, Transform, Vec2, With,
    },
    sprite::{Sprite, SpriteBundle},
    transform::TransformBundle,
};
use bevy_rapier2d::prelude::{
    Collider, GravityScale, LockedAxes, RigidBody, Velocity,
};

use crate::resources::game_config::GameConfig;
use crate::resources::game_state::GameState;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<MovePlayerEvent>()
            .add_event::<PlayerJumpEvent>()
            .add_startup_system(setup)
            .add_system(keyboard_events)
            .add_system(move_player_event_system)
            .add_system(move_player_system)
            .add_system(jump_player_event_system)
            .add_system(state_management_system)
            .add_system(flip_player_system);
    }
}

#[derive(PartialEq)]
enum ViewDirection {
    Left,
    Right,
}

#[derive(Component)]
struct Player {
    id: usize,
    acceleration: Vec2,
    is_jumping: bool,
    is_double_jumping: bool,
    is_falling: bool,
    view_direction: ViewDirection,
}

struct MovePlayerEvent {
    player_id: usize,
    translation: Vec2,
}

struct PlayerJumpEvent {
    player_id: usize,
    translation: Vec2,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_config: Res<GameConfig>,
    game_state: Res<GameState>,
) {
    if game_config.player_max_count > game_state.current_player_count {
        commands
            .spawn()
            .insert(Player {
                id: 1,
                acceleration: Vec2 { x: 0.0, y: 0.0 },
                is_jumping: false,
                is_double_jumping: false,
                is_falling: false,
                view_direction: ViewDirection::Left,
            })
            .insert(RigidBody::Dynamic)
            .insert(Velocity::default())
            .insert(GravityScale(10.0))
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(Collider::capsule(
                Vec2::new(0.0, 10.0),
                Vec2::new(0.0, 0.0),
                50.0,
            ))
            .insert_bundle(SpriteBundle {
                texture: asset_server.load("axe_idle.png"),
                ..Default::default()
            })
            .insert_bundle(TransformBundle::from(Transform::from_xyz(
                0.0, 0.0, 0.0,
            )));
    }
}

fn keyboard_events(
    mut keyboard_input_evr: EventReader<KeyboardInput>,
    mut move_player_event_wr: EventWriter<MovePlayerEvent>,
    mut player_jump_event_wr: EventWriter<PlayerJumpEvent>,
    game_config: Res<GameConfig>,
) {
    use bevy::input::ButtonState;

    for keyboard_input in keyboard_input_evr.iter() {
        match keyboard_input.state {
            ButtonState::Pressed => match keyboard_input.key_code {
                Some(KeyCode::A) => {
                    move_player_event_wr.send(MovePlayerEvent {
                        player_id: 1,
                        translation: Vec2 {
                            x: -game_config
                                .player_movement_translation_vector
                                .x
                                * game_config.player_acceleration.x,
                            y: 0.0,
                        },
                    })
                }
                Some(KeyCode::D) => {
                    move_player_event_wr.send(MovePlayerEvent {
                        player_id: 1,
                        translation: Vec2 {
                            x: game_config.player_movement_translation_vector.x
                                * game_config.player_acceleration.x,
                            y: 0.0,
                        },
                    })
                }
                Some(KeyCode::W) => {
                    player_jump_event_wr.send(PlayerJumpEvent {
                        player_id: 1,
                        translation: Vec2 {
                            x: 0.0,
                            y: game_config.player_movement_translation_vector.y
                                * game_config.player_acceleration.y,
                        },
                    })
                }
                _ => (),
            },
            ButtonState::Released => match keyboard_input.key_code {
                Some(KeyCode::A) => {
                    move_player_event_wr.send(MovePlayerEvent {
                        player_id: 1,
                        translation: Vec2 { x: 0.0, y: 0.0 },
                    })
                }
                Some(KeyCode::D) => {
                    move_player_event_wr.send(MovePlayerEvent {
                        player_id: 1,
                        translation: Vec2 { x: 0.0, y: 0.0 },
                    })
                }
                _ => (),
            },
        }
    }
}

fn move_player_event_system(
    mut move_player_evr: EventReader<MovePlayerEvent>,
    mut query: Query<&mut Player>,
) {
    for move_player_event in move_player_evr.iter() {
        for (mut player) in query.iter_mut() {
            if move_player_event.player_id == player.id {
                player.acceleration.x = move_player_event.translation.x;
            }
        }
    }
}

fn move_player_system(mut query: Query<(&mut Player, &mut Velocity)>) {
    for (mut player, mut velocity) in query.iter_mut() {
        if player.acceleration.x < 0.0 {
            player.view_direction = ViewDirection::Left
        } else if player.acceleration.x > 0.0 {
            player.view_direction = ViewDirection::Right
        }

        velocity.linvel.x = player.acceleration.x;
    }
}

fn state_management_system(mut query: Query<(&mut Player, &Velocity)>) {
    for (mut player, velocity) in query.iter_mut() {
        if velocity.linvel.y == 0.0 {
            player.is_jumping = false;
            player.is_double_jumping = false;
        }
        if velocity.linvel.y < 0.0 {
            player.is_falling = true;
        }
        if velocity.linvel.y >= 0.0 {
            player.is_falling = false;
        }
    }
}

fn jump_player_event_system(
    mut player_jump_evr: EventReader<PlayerJumpEvent>,
    mut query: Query<(&mut Player, &mut Velocity)>,
) {
    for (mut player, mut velocity) in query.iter_mut() {
        if !player.is_jumping {
            for player_jump_event in player_jump_evr.iter() {
                if player_jump_event.player_id == player.id {
                    velocity.linvel.y = player_jump_event.translation.y;
                    player.is_jumping = true;
                }
            }
        }

        if player.is_jumping && !player.is_double_jumping {
            for player_jump_event in player_jump_evr.iter() {
                if player_jump_event.player_id == player.id {
                    velocity.linvel.y = player_jump_event.translation.y;
                    player.is_double_jumping = true;
                }
            }
        }
    }
}

fn flip_player_system(mut query: Query<(&Player, &mut Sprite)>) {
    for (player, mut sprite) in query.iter_mut() {
        if player.view_direction == ViewDirection::Left {
            sprite.flip_x = false;
        } else if player.view_direction == ViewDirection::Right {
            sprite.flip_x = true;
        }
    }
}
