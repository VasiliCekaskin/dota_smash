use bevy::prelude::*;
use bevy_egui::{egui::Window, EguiContext, EguiPlugin};
use ggrs::NetworkStats;

#[derive(Default, Clone)]
pub struct Logger {
    log_lines: Vec<String>,
}

impl Logger {
    pub fn info(&mut self, msg: String) {
        self.log_lines.push("[INFO] ".to_string() + &msg);
    }
}

pub struct DebugUiPlugin;

impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Logger::default())
            .add_plugin(EguiPlugin)
            .add_system(ui_example);
    }
}
fn ui_example(
    mut egui_context: ResMut<EguiContext>,
    logger: Res<Logger>,
    network_status: Option<Res<NetworkStats>>,
) {
    Window::new("Debug").show(egui_context.ctx_mut(), |ui| {
        for log_line in logger.log_lines.iter().rev().take(5) {
            ui.label(log_line);
        }

        if network_status.is_some() {
            let network_status = network_status.unwrap();

            ui.label(
                "kbps_sent: ".to_string()
                    + &network_status.kbps_sent.to_string(),
            );

            ui.label(
                "local_frames_behind: ".to_string()
                    + &network_status.local_frames_behind.to_string(),
            );

            ui.label("ping: ".to_string() + &network_status.ping.to_string());
        }
    });
}
