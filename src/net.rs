use bevy::{
    prelude::{
        info, App, Commands, Component, ResMut, Schedule, Stage, SystemStage,
        Transform,
    },
    reflect::{self, Reflect},
    tasks::IoTaskPool,
};
use bevy_ggrs::{GGRSPlugin, SessionType};
use bevy_rapier2d::prelude::Velocity;
use bytemuck::{Pod, Zeroable};
use ggrs::{Config, PlayerType, SessionBuilder};
use matchbox_socket::WebRtcSocket;

use crate::game::{self, GameStage, GameState, ROLLBACK_DEFAULT};
use crate::player;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct BoxInput {
    pub inp: u8,
}

#[derive(Debug)]
pub struct GGRSConfig;
impl Config for GGRSConfig {
    type Input = BoxInput;
    type State = u8;
    type Address = String;
}
#[derive(Default, Reflect, Hash, Component)]
#[reflect(Hash)]
pub struct FrameCount {
    pub frame: u32,
}

pub fn setup_ggrs(mut app: &mut App) {
    GGRSPlugin::<GGRSConfig>::new()
        .with_update_frequency(game::FPS as usize)
        .with_input_system(player::input)
        .register_rollback_type::<Transform>()
        .register_rollback_type::<Velocity>()
        .with_rollback_schedule(
            Schedule::default().with_stage(
                ROLLBACK_DEFAULT,
                SystemStage::parallel()
                    .with_system(player::move_player_system)
                    .with_system(increase_frame_system),
            ),
        )
        .build(&mut app);
}

pub fn setup_socket(mut commands: Commands, mut game_state: ResMut<GameState>) {
    if game_state.stage != GameStage::Init {
        return; // Nothing to do we are not in the init phase
    }

    game_state.stage = GameStage::SetupSocket; // Go into SetupSocket stage

    let room_url = "ws://192.168.2.170:3536/next_2";

    info!("Connecting to matchbox server: {:?}", room_url);

    let (socket, message_loop) = WebRtcSocket::new(room_url);

    // The message loop needs to be awaited, or nothing will happen.
    // We do this here using bevy's task system.
    IoTaskPool::get().spawn(message_loop).detach();

    commands.insert_resource(Some(socket));

    game_state.stage = GameStage::SetupSession; // Done creating socket, create session
}

pub fn setup_session(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut socket: ResMut<Option<WebRtcSocket>>,
) {
    if game_state.stage != GameStage::SetupSession {
        return;
    }

    let socket = socket.as_mut();

    // Check for new connections
    socket.as_mut().unwrap().accept_new_connections();

    let players = socket.as_ref().unwrap().players();

    let num_players = 2;
    if players.len() < num_players {
        return; // wait for more playere
    }

    info!("All peers have joined, going in-game");
    // consume the socket (currently required because GGRS takes ownership of its socket)
    let socket = socket.take().unwrap();

    // create a GGRS P2P session
    let mut session_builder = SessionBuilder::<GGRSConfig>::new()
        .with_num_players(num_players)
        .with_max_prediction_window(12)
        .with_fps(60 as usize)
        .expect("Invalid FPS")
        .with_input_delay(2);

    let mut handles = Vec::new();
    for (i, player_type) in socket.players().iter().enumerate() {
        if *player_type == PlayerType::Local {
            handles.push(i);
        }
        session_builder = session_builder
            .add_player(player_type.clone(), i)
            .expect("Invalid player added.");
    }

    // start the GGRS session
    let session = session_builder.start_p2p_session(socket).unwrap();

    commands.insert_resource(session);
    commands.insert_resource(SessionType::P2PSession);

    game_state.stage = GameStage::SpawnPlayers;
}

pub fn increase_frame_system(mut frame_count: ResMut<FrameCount>) {
    frame_count.frame += 1;
}
