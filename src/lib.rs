use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::{App, Schedule, SystemStage, Transform};
use bevy::window::WindowDescriptor;
use bevy::DefaultPlugins;
use bevy_ggrs::GGRSPlugin;

mod game;
mod net;
mod player;

use game::{GameStage, GameState};
use net::GGRSConfig;

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;
const FPS: f32 = 60.0;
const ROLLBACK_DEFAULT: &str = "rollback_default";

pub const LAUNCHER_TITLE: &str = "Dota Smash";

pub fn app() -> App {
    let mut app = App::new();

    GGRSPlugin::<GGRSConfig>::new()
        .with_update_frequency(FPS as usize)
        .with_input_system(player::input)
        .register_rollback_type::<Transform>()
        .with_rollback_schedule(
            Schedule::default().with_stage(
                ROLLBACK_DEFAULT,
                SystemStage::parallel()
                    .with_system(player::move_player_system)
                    .with_system(net::increase_frame_system),
            ),
        )
        .build(&mut app);

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
    // .add_plugin(LogDiagnosticsPlugin::default())
    // .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_system(bevy::window::close_on_esc)
    .add_startup_system(net::setup_socket)
    .add_system(net::setup_session)
    .add_system(player::setup_players)
    .add_system(player::animate_players)
    .run();

    return app;
}
