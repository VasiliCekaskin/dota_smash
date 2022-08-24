use bevy::prelude::*;
mod menu;
mod net;

//// My own crates
mod dota_smash;
use dota_smash::DotaSmashPlugin;
///

pub const LAUNCHER_TITLE: &str = "Dota Smash";

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;

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
    .add_plugins(DefaultPlugins)
    // .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
    // .add_plugin(RapierDebugRenderPlugin::default())
    .add_plugin(DotaSmashPlugin)
    .run();

    return app;
}
