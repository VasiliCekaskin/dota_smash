use bevy::{
    input::keyboard::KeyboardInput,
    prelude::{
        AssetServer, Commands, Component, EventReader, EventWriter, KeyCode,
        Plugin, Query, Res, Transform, Vec2, With,
    },
    sprite::SpriteBundle,
    transform::TransformBundle,
};
use bevy_rapier2d::prelude::{Collider, GravityScale, LockedAxes, RigidBody};

use crate::{GameConfig, GameState};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<MovePlayerEvent>()
            .add_startup_system(setup)
            .add_system(keyboard_events)
            .add_system(move_player_event_system)
            .add_system(move_player_system);
    }
}

#[derive(Component)]
struct Player {
    id: usize,
    acceleration: Vec2,
}

struct MovePlayerEvent {
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
            })
            .insert(RigidBody::Dynamic)
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
    mut move_player_event: EventWriter<MovePlayerEvent>,
    game_config: Res<GameConfig>,
) {
    use bevy::input::ButtonState;

    for keyboard_input in keyboard_input_evr.iter() {
        match keyboard_input.state {
            ButtonState::Pressed => match keyboard_input.key_code {
                Some(KeyCode::A) => move_player_event.send(MovePlayerEvent {
                    player_id: 1,
                    translation: Vec2 {
                        x: -game_config.player_movement_translation_vector.x
                            * game_config.player_acceleration.x,
                        y: 0.0,
                    },
                }),
                Some(KeyCode::D) => move_player_event.send(MovePlayerEvent {
                    player_id: 1,
                    translation: Vec2 {
                        x: game_config.player_movement_translation_vector.x
                            * game_config.player_acceleration.x,
                        y: 0.0,
                    },
                }),
                _ => (),
            },
            ButtonState::Released => match keyboard_input.key_code {
                Some(KeyCode::A) => move_player_event.send(MovePlayerEvent {
                    player_id: 1,
                    translation: Vec2 { x: 0.0, y: 0.0 },
                }),
                Some(KeyCode::D) => move_player_event.send(MovePlayerEvent {
                    player_id: 1,
                    translation: Vec2 { x: 0.0, y: 0.0 },
                }),
                _ => (),
            },
        }
    }
}

fn move_player_event_system(
    mut move_player_event_evr: EventReader<MovePlayerEvent>,
    mut query: Query<&mut Player>,
) {
    for move_player_event in move_player_event_evr.iter() {
        for (mut player) in query.iter_mut() {
            if move_player_event.player_id == player.id {
                player.acceleration = move_player_event.translation;
            }
        }
    }
}

// TODO move the player according to player acceleration
fn move_player_system(mut query: Query<(&Player, &mut Transform)>) {
    for (player, mut transform) in query.iter_mut() {
        transform.translation.x += player.acceleration.x;
        transform.translation.y += player.acceleration.y;
    }
}
