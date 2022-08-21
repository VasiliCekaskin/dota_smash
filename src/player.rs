use bevy::{
    prelude::{
        info, AssetServer, Assets, Camera2dBundle, Commands, Component, Deref,
        DerefMut, Handle, In, Input, KeyCode, OrthographicProjection, Query,
        Res, ResMut, Transform, Vec2, Vec3, With,
    },
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    time::{Time, Timer},
    transform::TransformBundle,
};
use bevy_ggrs::{Rollback, RollbackIdProvider};
use ggrs::{InputStatus, P2PSession, PlayerHandle};

use crate::{
    net::{BoxInput, GGRSConfig},
    GameStage, GameState,
};

const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct Player {
    pub handle: usize,
}

pub fn setup_players(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut rip: ResMut<RollbackIdProvider>,
    session: Option<ResMut<P2PSession<GGRSConfig>>>,
    mut game_state: ResMut<GameState>,
) {
    if game_state.stage != GameStage::SpawnPlayers {
        return;
    }

    if session.is_none() {
        info!("Session is none...");
        return;
    }

    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 2.0,
            ..Default::default()
        },
        ..Default::default()
    });

    let num_players = session.unwrap().num_players();

    for handle in 0..num_players {
        let texture_handle = asset_server.load("venomancer_idle.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(100.0, 100.0),
            5,
            1,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let mut transform = Transform::default();

        match handle {
            0 => transform = Transform::from_xyz(-100.0, 0.0, 0.0),
            1 => transform = Transform::from_xyz(100.0, 0.0, 0.0),
            _ => (),
        }

        commands
            .spawn()
            .insert(Player { handle })
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                ..Default::default()
            })
            .insert(AnimationTimer(Timer::from_seconds(0.15, true)))
            .insert_bundle(TransformBundle::from(transform.with_scale(Vec3 {
                x: 2.0,
                y: 2.0,
                z: 1.0,
            })))
            .insert(Rollback::new(rip.next_id()));
    }

    game_state.stage = GameStage::Gameplay;
}

pub fn animate_players(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &Player,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (player, mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas =
                texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

pub fn input(
    _handle: In<PlayerHandle>,
    keyboard_input: Res<Input<KeyCode>>,
) -> BoxInput {
    let mut input: u8 = 0;

    if keyboard_input.pressed(KeyCode::W) {
        input |= INPUT_UP;
    }
    if keyboard_input.pressed(KeyCode::A) {
        input |= INPUT_LEFT;
    }
    if keyboard_input.pressed(KeyCode::S) {
        input |= INPUT_DOWN;
    }
    if keyboard_input.pressed(KeyCode::D) {
        input |= INPUT_RIGHT;
    }

    BoxInput { inp: input }
}

pub fn move_player_system(
    mut query: Query<(&mut Transform, &Player), With<Rollback>>,
    inputs: Res<Vec<(BoxInput, InputStatus)>>,
) {
    for (mut t, p) in query.iter_mut() {
        let input = inputs[p.handle as usize].0.inp;

        if input & INPUT_LEFT != 0 && input & INPUT_RIGHT == 0 {
            t.translation.x -= 50.;
        }
        if input & INPUT_LEFT == 0 && input & INPUT_RIGHT != 0 {
            t.translation.x += 50.;
        }
    }
}
