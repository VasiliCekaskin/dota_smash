use bevy_rapier2d::prelude::*;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::camera::{CameraProjection, Projection, ScalingMode, Viewport},
};

mod entities;
use entities::player::PlayerPlugin;

mod resources;
use resources::game_config::GameConfig;
use resources::game_state::GameState;

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;

#[derive(Component)]
struct Platform;

pub const LAUNCHER_TITLE: &str = "Dota Smash";

pub fn app() -> App {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: LAUNCHER_TITLE.to_string(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        canvas: Some("#bevy".to_string()),
        fit_canvas_to_parent: true,
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
    .insert_resource(GameState::default())
    .insert_resource(GameConfig::default())
    .add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0))
    .add_plugin(RapierDebugRenderPlugin::default())
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_startup_system(setup)
    .add_plugin(PlayerPlugin)
    .add_system(bevy::window::close_on_esc)
    .run();

    return app;
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera_2d_bundle = Camera2dBundle::default();

    camera_2d_bundle.projection.scale = 3.0;

    commands.spawn_bundle(camera_2d_bundle);

    // ground
    commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(WINDOW_WIDTH, 50.0))
        .insert(Friction {
            coefficient: 0.0,
            ..Default::default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            0.0, 2200.0, 0.0,
        )));

    // Platform
    commands
        .spawn()
        .insert(Platform)
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(200.0, 50.0))
        .insert(Friction {
            coefficient: 0.0,
            ..Default::default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            0.0, -200.0, 0.0,
        )));

    commands
        .spawn()
        .insert(Platform)
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(100.0, 20.0))
        .insert(Friction {
            coefficient: 0.0,
            ..Default::default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            -300.0, 80.0, 0.0,
        )));

    commands
        .spawn()
        .insert(Platform)
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(100.0, 20.0))
        .insert(Friction {
            coefficient: 0.0,
            ..Default::default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            -300.0, 230.0, 0.0,
        )));

    commands
        .spawn()
        .insert(Platform)
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(100.0, 20.0))
        .insert(Friction {
            coefficient: 0.0,
            ..Default::default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            400.0, -50.0, 0.0,
        )));
}
