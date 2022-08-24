use bevy::prelude::*;
use bevy_egui::*;
use ggrs::P2PSession;
use iyes_loopless::prelude::*;

use crate::{dota_smash::*, net::prelude::GGRSConfig};
pub struct OnlineLobbyMenuPlugin;

impl Plugin for OnlineLobbyMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ui.run_if(game_stage_is_online_lobby_menu));
    }
}

fn ui(
    mut game_state: ResMut<GameState>,
    session: Option<ResMut<P2PSession<GGRSConfig>>>,
    mut egui_ctx: ResMut<EguiContext>,
) {
    let mut online_button_clicked = false;
    let mut start_game_button_clicked = false;

    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(420.0);
            ui.heading("Online lobby");

            let mut num_players = 0;

            if session.is_some() {
                let sess = session.unwrap();

                for i in sess.local_player_handles().iter() {
                    num_players += 1;
                    ui.label("You: ".to_owned() + i.to_string().as_str());
                }

                for i in sess.remote_player_handles().iter() {
                    num_players += 2;
                    ui.label(
                        "Remote Player: ".to_owned() + i.to_string().as_str(),
                    );
                }
            }

            ui.add_space(20.0);
            online_button_clicked = ui
                .add_sized(
                    egui::Vec2::new(100.0, 20.0),
                    egui::Button::new("Go back..."),
                )
                .clicked();

            if num_players >= 2 {
                ui.add_space(20.0);
                start_game_button_clicked = ui
                    .add_sized(
                        egui::Vec2::new(100.0, 20.0),
                        egui::Button::new("Start Game!"),
                    )
                    .clicked()
            }
        });
    });

    if online_button_clicked {
        game_state.game_stage = GameStage::MainMenu
    }
}

fn game_stage_is_online_lobby_menu(game_state: Res<GameState>) -> bool {
    if game_state.game_stage == GameStage::OnlineLobbyMenu {
        return true;
    } else {
        return false;
    }
}
