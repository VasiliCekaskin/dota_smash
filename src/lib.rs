use bevy_rapier2d::prelude::*;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

mod entities;
use entities::player::PlayerPlugin;

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;

////////////////////////////////////////////////////////////////
// This must come into it's own modules later

const PLAYER_MAX_COUNT: u32 = 2;
const PLAYER_MOVEMENT_TRANSLATION_VECTOR: Vec2 = Vec2 { x: 1.0, y: 1.0 };
const PLAYER_MAX_MOVEMENT_SPEED: f32 = 20.0;
const PLAYER_ACCELERATION: Vec2 = Vec2 {
    x: PLAYER_MAX_MOVEMENT_SPEED,
    y: 0.0,
};

struct GameConfig {
    player_max_count: u32,
    player_movement_translation_vector: Vec2,
    player_max_movement_speed: f32,
    player_acceleration: Vec2,
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

struct GameState {
    current_player_count: u32,
}
impl Default for GameState {
    fn default() -> Self {
        Self {
            current_player_count: 0,
        }
    }
}
////////////////////////////////////////////////////////////////

#[derive(Component)]
struct Platform;

#[derive(Component)]
struct Speed {
    value: f32,
}

#[derive(Component)]
struct JumpSpeed {
    value: f32,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum StatusTypes {
    Idle,
    RunLeft,
    RunRight,
    Jumping,
    Deceleration,
}
#[derive(Component)]
struct Statuses {
    value: Vec<StatusTypes>,
}

impl Statuses {
    fn add_status(&mut self, status_type: &StatusTypes) {
        self.value.push(*status_type);
        self.value.dedup();
    }

    fn remove_status(&mut self, status_type: &StatusTypes) {
        if let Some(index) =
            self.value.iter().position(|status| status == status_type)
        {
            self.value.swap_remove(index);
        }
    }
}

#[derive(PartialEq)]
enum ViewDirections {
    Left,
    Right,
}

#[derive(Component)]
struct ViewDirection {
    value: ViewDirections,
}

pub const LAUNCHER_TITLE: &str = "Bevy Shell - Template";

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
    commands.spawn_bundle(Camera2dBundle::default());

    // ground
    commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(WINDOW_WIDTH, 50.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            0.0, 2200.0, 0.0,
        )));

    // Platform
    commands
        .spawn()
        .insert(Platform)
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(200.0, 50.0))
        // .insert_bundle(SpriteBundle {
        //     texture: asset_server.load("sprites/platform.png"),
        //     ..default()
        // })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            0.0, -200.0, 0.0,
        )));

    commands
        .spawn()
        .insert(Platform)
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(100.0, 20.0))
        // .insert_bundle(SpriteBundle {
        //     texture: asset_server.load("sprites/platform.png"),
        //     ..default()
        // })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            -300.0, 80.0, 0.0,
        )));

    commands
        .spawn()
        .insert(Platform)
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(100.0, 20.0))
        // .insert_bundle(SpriteBundle {
        //     texture: asset_server.load("sprites/platform.png"),
        //     ..default()
        // })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            -300.0, 230.0, 0.0,
        )));

    commands
        .spawn()
        .insert(Platform)
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(100.0, 20.0))
        // .insert_bundle(SpriteBundle {
        //     texture: asset_server.load("sprites/platform.png"),
        //     ..default()
        // })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            400.0, -50.0, 0.0,
        )));
}
