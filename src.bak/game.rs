use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite::collide_aabb;
use bevy_ggrs::Rollback;
use bevy_ggrs::RollbackIdProvider;
use bevy_rapier2d::parry::query::details::intersection_test_aabb_segment;
use bevy_rapier2d::parry::query::intersection_test;
use bevy_rapier2d::prelude::*;

use crate::net;
use crate::player;
use crate::player::Player;

pub const FPS: f32 = 60.0;
pub const ROLLBACK_DEFAULT: &str = "rollback_default";

#[derive(PartialEq, Debug)]
pub enum GameStage {
    SetupLobby,
    SetupLobbyPlayer,
    SetupSocket,
    SetupSession,
    SetupGameplayPlayers,
    Gameplay,
}

#[derive(Debug)]
pub struct GameState {
    pub stage: GameStage,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        net::setup_ggrs(app);

        app.insert_resource(GameState {
            stage: GameStage::SetupLobby,
        })
        .add_system(setup_lobby)
        .add_system(player::setup_lobby_player)
        .add_system(player::local_input_system)
        .add_system(player::animate_players)
        .add_system(net::setup_socket)
        .add_system(net::setup_session)
        .add_system(player::setup_gameplay_players)
        .add_system(fireball_system)
        .add_system(animate_fireball_system);
    }
}

fn setup_lobby(
    commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut game_state: ResMut<GameState>,
) {
    if game_state.stage != GameStage::SetupLobby {
        return;
    }

    // setup camera, background & platform
    setup_world(commands, asset_server);

    // Lobby is set up transition to next game stage
    game_state.stage = GameStage::SetupLobbyPlayer;
}

fn setup_world(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    // Camera
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 2.0,
            ..Default::default()
        },
        ..Default::default()
    });

    // Background
    let background_texture = asset_server.load("background.png");

    commands
        .spawn_bundle(SpriteBundle {
            texture: background_texture,
            ..Default::default()
        })
        .insert(Transform::default().with_scale(Vec3::splat(2.0)));

    let platform_texture = asset_server.load("platform.png");

    // Platform
    commands
        .spawn_bundle(SpriteBundle {
            texture: platform_texture,
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Friction {
            coefficient: 0.0,
            ..Default::default()
        })
        .insert(Collider::cuboid(400.0, 75.0))
        .insert_bundle(TransformBundle::from(
            Transform::from_xyz(0.0, -400.0, 1.0).with_scale(Vec3::splat(2.0)),
        ));
}

#[derive(Component, Reflect, Default)]
pub struct Fireball {
    player_handle: usize,
}

#[derive(Component)]
pub struct FireballTimer(Timer);

#[derive(Component)]
pub struct FireballLiveTimer(Timer);

#[derive(Component)]
pub struct FireballAnimationTimer(Timer);

pub fn spawn_fireball(
    commands: &mut Commands,
    mut texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    player_entity: (&Player, &Transform, &TextureAtlasSprite),
    asset_server: &AssetServer,
    rip: &mut RollbackIdProvider,
    fireball_query: &Query<(&Fireball, &FireballTimer)>,
) {
    for (f, ft) in fireball_query {
        if f.player_handle == player_entity.0.handle {
            if !ft.0.finished() {
                return; // Any fireball fired recently? Don't fire a new one...
            }
        }
    }

    let texture_handle = asset_server.load("fireball.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(25.0, 25.0), 6, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut transform = player_entity.1.clone();
    let mut vel = Velocity::default();
    if player_entity.2.flip_x {
        vel.linvel.x = -30.0;
        transform.translation.x -= 50.0;
    } else {
        vel.linvel.x = 30.0;
        transform.translation.x += 50.0;
    }

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            ..Default::default()
        })
        .insert(Fireball {
            player_handle: player_entity.0.handle,
            ..Default::default()
        })
        .insert(FireballAnimationTimer(Timer::from_seconds(0.05, true)))
        .insert(FireballTimer(Timer::from_seconds(0.2, false)))
        .insert(FireballLiveTimer(Timer::from_seconds(3.0, false)))
        .insert(transform)
        .insert(vel)
        .insert(Rollback::new(rip.next_id()));
}

fn fireball_system(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &Fireball,
            &Velocity,
            &mut Transform,
            &mut FireballTimer,
            &mut FireballLiveTimer,
        ),
        Without<Player>,
    >,
    mut player_query: Query<
        (&Player, &Velocity, &mut Transform),
        Without<Fireball>,
    >,
    time: Res<Time>,
) {
    for (e, f, v, mut t, mut ft, mut flt) in query.iter_mut() {
        for (p, p_v, mut p_t) in player_query.iter_mut() {
            if p.handle != f.player_handle {
                let collision = bevy::sprite::collide_aabb::collide(
                    t.translation,
                    Vec2::new(25.0, 25.0),
                    p_t.translation,
                    Vec2::new(100.0, 100.0),
                );

                if collision.is_some() {
                    p_t.translation.x += v.linvel.x;
                    commands.entity(e).despawn();
                }
            }
        }

        t.translation.x += v.linvel.x;
        ft.0.tick(Duration::from_millis(time.delta().as_millis() as u64));
        flt.0
            .tick(Duration::from_millis(time.delta().as_millis() as u64));
        if flt.0.finished() {
            commands.entity(e).despawn();
        }
    }
}

pub fn animate_fireball_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &Fireball,
        &mut FireballAnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (_, mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            let texture_atlas =
                texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}
