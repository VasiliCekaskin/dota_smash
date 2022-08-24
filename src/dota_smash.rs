use bevy::prelude::*;

use crate::{menu::prelude::MenuPlugin, net::prelude::NetPlugin};

#[derive(PartialEq)]
pub enum GameStage {
    MainMenu,
    OnlineLobbyMenu,
}

#[derive(PartialEq)]
pub struct GameState {
    pub game_stage: GameStage,
}

pub struct DotaSmashPlugin;

impl Plugin for DotaSmashPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState {
            game_stage: GameStage::MainMenu,
        })
        .add_plugin(MenuPlugin)
        .add_plugin(NetPlugin);
    }
}
