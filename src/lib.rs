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
    .add_startup_system(net::setup_socket)
    .add_system(net::setup_session)
    .add_system(player::setup_players)
    .add_system(player::animate_players)
    .run();

    return app;
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad {
                size: Vec2::new(2000.0, 20.0),
                ..Default::default()
            })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Friction {
            coefficient: 0.0,
            ..Default::default()
        })
        .insert(Collider::cuboid(20000.0, 20.0));
}
