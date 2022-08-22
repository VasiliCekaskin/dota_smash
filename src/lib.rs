use bevy::asset::AssetServerError;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::prelude::{App, Commands};
use bevy::window::WindowDescriptor;
use bevy::DefaultPlugins;

mod game;
mod net;
mod player;

use bevy_rapier2d::prelude::*;
use game::{GameStage, GameState};

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;

pub const LAUNCHER_TITLE: &str = "Dota Smash";

pub fn app() -> App {
    let mut app = App::new();

    net::setup_ggrs(&mut app);

    app.insert_resource(WindowDescriptor {
        title: LAUNCHER_TITLE.to_string(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        canvas: Some("#bevy".to_string()),
        fit_canvas_to_parent: true,
        ..Default::default()
    })
    .insert_resource(GameState {
        stage: GameStage::Init,
    })
    .insert_resource(net::FrameCount { frame: 0 })
    .add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
    .add_plugin(RapierDebugRenderPlugin::default())
    // .add_plugin(LogDiagnosticsPlugin::default())
    // .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_system(bevy::window::close_on_esc)
    .add_startup_system(setup_world)
    // .add_startup_system(net::setup_socket)
    // .add_system(net::setup_session)
    // .add_system(player::setup_players)
    // .add_system(player::animate_players)
    .run();

    return app;
}

fn setup_world(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
