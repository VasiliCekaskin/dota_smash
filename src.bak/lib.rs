use bevy::prelude::*;
use bevy_inspector_egui::*;
use bevy_rapier2d::prelude::*;

mod debug_ui;
mod game;
mod menu;
mod net;
mod player;

use game::*;
use player::*;

pub const LAUNCHER_TITLE: &str = "Dota Smash";

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;

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
    .add_plugins(DefaultPlugins)
    .add_plugin(debug_ui::DebugUiPlugin)
    .add_plugin(WorldInspectorPlugin::new())
    .register_inspectable::<Player>()
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
    .add_plugin(RapierDebugRenderPlugin::default())
    .add_plugin(GamePlugin)
    // .add_plugin(menu::MenuPlugin)
    // .add_startup_system(net::setup_socket)
    // .add_system(net::setup_session)
    .run();

    return app;
}
