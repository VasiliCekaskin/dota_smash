use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::net;
use crate::player;

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
        .add_system(player::setup_gameplay_players);
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
