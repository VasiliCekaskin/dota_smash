use bevy::{
    prelude::{
        info, AssetServer, Assets, BuildChildren, Camera2dBundle, Commands,
        Component, Deref, DerefMut, Handle, In, Input, KeyCode,
        OrthographicProjection, Query, Res, ResMut, Transform, Vec2, Vec3,
        With,
    },
    sprite::{Sprite, SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    time::{Time, Timer},
    transform::TransformBundle,
};
use bevy_ggrs::{Rollback, RollbackIdProvider};
use bevy_rapier2d::prelude::{
    Collider, CollisionGroups, Friction, GravityScale, LockedAxes, RigidBody,
    SolverGroups, Velocity,
};
use ggrs::{InputStatus, P2PSession, PlayerHandle};

use crate::{
    net::{BoxInput, GGRSConfig},
    GameStage, GameState,
};

const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;

const PLAYER_SPEED: f32 = 400.;

const PLAYER_COLLISION_GROUP: u32 = 0b01;
const OTHER_COLLISION_GROUP: u32 = 0b10;

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
            0 => transform = Transform::from_xyz(-100.0, 80.0, 0.0),
            1 => transform = Transform::from_xyz(100.0, 80.0, 0.0),
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
            .insert(RigidBody::Dynamic)
            .insert(Friction {
                coefficient: 0.0,
                ..Default::default()
            })
            .insert(Velocity::default())
            .insert(GravityScale(10.0))
            .insert(LockedAxes::ROTATION_LOCKED)
            .with_children(|children| {
                children
                    .spawn()
                    .insert(Collider::cuboid(25.0, 30.0))
                    .insert(CollisionGroups::new(
                        PLAYER_COLLISION_GROUP,
                        OTHER_COLLISION_GROUP,
                    ))
                    .insert_bundle(TransformBundle::from(Transform::from_xyz(
                        0.0, -40.0, 0.0,
                    )));
            })
            // .insert(CollisionGroups::new(0b0001, 0b0010))
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
    for (_, mut timer, mut sprite, texture_atlas_handle) in &mut query {
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
    mut query: Query<
        (&Player, &mut Velocity, &mut TextureAtlasSprite),
        With<Rollback>,
    >,
    inputs: Res<Vec<(BoxInput, InputStatus)>>,
) {
    for (p, mut v, mut s) in query.iter_mut() {
        let input = inputs[p.handle as usize].0.inp;

        if input & INPUT_LEFT != 0 && input & INPUT_RIGHT == 0 {
            s.flip_x = true;
            v.linvel.x = -PLAYER_SPEED;
        }
        if input & INPUT_LEFT == 0 && input & INPUT_RIGHT != 0 {
            s.flip_x = false;
            v.linvel.x = PLAYER_SPEED;
        }
        if input & INPUT_UP != 0 {
            v.linvel.y = PLAYER_SPEED;
        }

        if input == 0 {
            v.linvel.x = 0.;
        }
    }
}
