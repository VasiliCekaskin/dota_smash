use bevy::prelude::*;
use bevy_egui::*;
use iyes_loopless::prelude::*;

use crate::dota_smash::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ui.run_if(game_stage_is_main_menu));
    }
}

fn ui(mut game_state: ResMut<GameState>, mut egui_ctx: ResMut<EguiContext>) {
    let mut online_button_clicked = false;

    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(340.0);
            ui.heading("Menu");
            ui.add_space(20.0);
            online_button_clicked = ui
                .add_sized(
                    egui::Vec2::new(100.0, 20.0),
                    egui::Button::new("Play online"),
                )
                .clicked();
        });
    });

    if online_button_clicked {
        game_state.game_stage = GameStage::OnlineLobbyMenu
    }
}
fn game_stage_is_main_menu(game_state: Res<GameState>) -> bool {
    if game_state.game_stage == GameStage::MainMenu {
        return true;
    } else {
        return false;
    }
}
