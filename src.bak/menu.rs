use bevy::prelude::*;
use bevy_egui::{egui::Window, EguiContext, EguiPlugin};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin).add_system(ui_example);
    }
}
fn ui_example(mut egui_context: ResMut<EguiContext>) {
    Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}
