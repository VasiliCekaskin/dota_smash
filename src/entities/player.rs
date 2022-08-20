use crate::fireball::Fireball;
use bevy::{
    input::keyboard::KeyboardInput,
    prelude::Timer,
    prelude::{
        AssetServer, Assets, Commands, Component, Deref, DerefMut, EventReader,
        EventWriter, Handle, KeyCode, Plugin, Query, Res, ResMut, Transform,
        Vec2, Vec3,
    },
    sprite::{
        Sprite, SpriteBundle, SpriteSheetBundle, TextureAtlas,
        TextureAtlasSprite,
    },
    time::Time,
    transform::TransformBundle,
};
use bevy_rapier2d::prelude::{
    Collider, Friction, GravityScale, LockedAxes, RigidBody, Velocity,
};

use crate::resources::game_config::GameConfig;
use crate::resources::game_state::GameState;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<MovePlayerEvent>()
            .add_event::<PlayerJumpEvent>()
            .add_event::<PlayerAttackEvent>()
            .add_startup_system(setup)
            .add_system(keyboard_events)
            .add_system(move_player_event_system)
            .add_system(move_player_system)
            .add_system(jump_player_event_system)
            .add_system(state_management_system)
            .add_system(attack_player_event_system)
            .add_system(flip_player_system)
            .add_system(animate);
    }
}

#[derive(PartialEq)]
pub enum ViewDirection {
    Left,
    Right,
}

pub enum AnimationState {
    Idle,
    Running,
}

#[derive(Component)]
pub struct Player {
    pub id: usize,
    acceleration: Vec2,
    is_jumping: bool,
    is_double_jumping: bool,
    is_falling: bool,
    pub view_direction: ViewDirection,
    pub animation_state: AnimationState,
}

struct MovePlayerEvent {
    player_id: usize,
    translation: Vec2,
}

struct PlayerJumpEvent {
    player_id: usize,
    translation: Vec2,
}

struct PlayerAttackEvent {
    player_id: usize,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    game_config: Res<GameConfig>,
    game_state: Res<GameState>,
) {
    let texture_handle = asset_server.load("venomancer_running.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(100.0, 100.0), 5, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

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
                animation_state: AnimationState::Idle,
            })
            .insert(RigidBody::Dynamic)
            .insert(Friction {
                coefficient: 0.0,
                ..Default::default()
            })
            .insert(Velocity::default())
            .insert(GravityScale(10.0))
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(Collider::cuboid(50.0, 50.0))
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                ..Default::default()
            })
            .insert(AnimationTimer(Timer::from_seconds(0.15, true)))
            .insert_bundle(TransformBundle::from(
                Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3 {
                    x: 2.0,
                    y: 2.0,
                    z: 1.0,
                }),
            ));
    }
}
#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas =
                texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn keyboard_events(
    mut keyboard_input_evr: EventReader<KeyboardInput>,
    mut move_player_ewr: EventWriter<MovePlayerEvent>,
    mut player_jump_ewr: EventWriter<PlayerJumpEvent>,
    mut player_attack_ewr: EventWriter<PlayerAttackEvent>,
    game_config: Res<GameConfig>,
) {
    use bevy::input::ButtonState;

    for keyboard_input in keyboard_input_evr.iter() {
        match keyboard_input.state {
            ButtonState::Pressed => match keyboard_input.key_code {
                Some(KeyCode::A) => move_player_ewr.send(MovePlayerEvent {
                    player_id: 1,
                    translation: Vec2 {
                        x: -game_config.player_movement_translation_vector.x
                            * game_config.player_acceleration.x,
                        y: 0.0,
                    },
                }),
                Some(KeyCode::D) => move_player_ewr.send(MovePlayerEvent {
                    player_id: 1,
                    translation: Vec2 {
                        x: game_config.player_movement_translation_vector.x
                            * game_config.player_acceleration.x,
                        y: 0.0,
                    },
                }),
                Some(KeyCode::W) => player_jump_ewr.send(PlayerJumpEvent {
                    player_id: 1,
                    translation: Vec2 {
                        x: 0.0,
                        y: game_config.player_movement_translation_vector.y
                            * game_config.player_acceleration.y,
                    },
                }),
                Some(KeyCode::Space) => {
                    player_attack_ewr.send(PlayerAttackEvent { player_id: 1 })
                }
                _ => (),
            },
            ButtonState::Released => match keyboard_input.key_code {
                Some(KeyCode::A) => move_player_ewr.send(MovePlayerEvent {
                    player_id: 1,
                    translation: Vec2 { x: 0.0, y: 0.0 },
                }),
                Some(KeyCode::D) => move_player_ewr.send(MovePlayerEvent {
                    player_id: 1,
                    translation: Vec2 { x: 0.0, y: 0.0 },
                }),
                _ => (),
            },
        }
    }
}

fn move_player_event_system(
    mut move_player_evr: EventReader<MovePlayerEvent>,
    mut query: Query<(&mut Player)>,
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

fn attack_player_event_system(
    mut commands: Commands,
    mut player_attack_er: EventReader<PlayerAttackEvent>,
    mut asset_server: ResMut<AssetServer>,
    query: Query<(&Player, &Transform)>,
) {
    for player_attack_event in player_attack_er.iter() {
        for (player, transform) in query.iter() {
            if (player_attack_event.player_id == player.id) {
                Fireball::new_for_player(
                    player,
                    transform,
                    &mut commands,
                    &mut asset_server,
                );
            }
        }
    }
}

fn flip_player_system(mut query: Query<(&Player, &mut TextureAtlasSprite)>) {
    for (player, mut sprite) in query.iter_mut() {
        if player.view_direction == ViewDirection::Left {
            sprite.flip_x = true;
        } else if player.view_direction == ViewDirection::Right {
            sprite.flip_x = false;
        }
    }
}
