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
use game::GamePlugin;

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;

pub const LAUNCHER_TITLE: &str = "Dota Smash";

pub fn app() -> App {
    let mut app = App::new();

    // net::setup_ggrs(&mut app);

    app.insert_resource(WindowDescriptor {
        title: LAUNCHER_TITLE.to_string(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        canvas: Some("#bevy".to_string()),
        fit_canvas_to_parent: true,
        ..Default::default()
    })
    .insert_resource(net::FrameCount { frame: 0 })
    .add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
    .add_plugin(RapierDebugRenderPlugin::default())
    .add_plugin(GamePlugin)
    // .add_startup_system(net::setup_socket)
    // .add_system(net::setup_session)
    .run();

    return app;
}
