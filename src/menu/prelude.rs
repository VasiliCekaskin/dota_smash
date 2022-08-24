use bevy::prelude::Plugin;
use bevy_egui::EguiPlugin;

use super::main_menu::MainMenuPlugin;
use super::online_lobby_menu::OnlineLobbyMenuPlugin;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(EguiPlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(OnlineLobbyMenuPlugin);
    }
}
