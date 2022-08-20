use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::{
    App, Camera2dBundle, ClearColor, Color, Commands, OrthographicProjection,
};
use bevy::window::WindowDescriptor;
use bevy::DefaultPlugins;
use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::render::RapierDebugRenderPlugin;

mod networking;

mod entities;

mod fireball;
use entities::player::PlayerPlugin;

mod resources;
use fireball::FireballPlugin;
use resources::game_config::GameConfig;
use resources::game_state::GameState;

mod maps;
use maps::map_one::MapOnePlugin;

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;

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
    .add_plugin(networking::setup::NetworkingPlugin)
    .add_plugin(ShapePlugin)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
    // .add_plugin(RapierDebugRenderPlugin::default())
    // .add_plugin(LogDiagnosticsPlugin::default())
    // .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_startup_system(setup_camera)
    .add_plugin(MapOnePlugin)
    .add_plugin(FireballPlugin)
    .add_plugin(PlayerPlugin)
    .add_system(bevy::window::close_on_esc)
    .run();

    return app;
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 3.0,
            ..Default::default()
        },
        ..Default::default()
    });
}
