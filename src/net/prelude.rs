use bevy::{
    prelude::{
        info, App, Commands, Component, In, Input, KeyCode, Plugin, Res,
        ResMut, Schedule, SystemStage,
    },
    reflect::Reflect,
    tasks::IoTaskPool,
};
use bevy_ggrs::{GGRSPlugin, SessionType};
use bytemuck::{Pod, Zeroable};
use ggrs::{Config, P2PSession, PlayerHandle, SessionBuilder};
use iyes_loopless::prelude::*;
use matchbox_socket::WebRtcSocket;

use crate::dota_smash::{GameStage, GameState};

const ROOM_URL: &str = "ws://192.168.2.170:3536/next_2";
const ROLLBACK_DEFAULT: &str = "rollback_default";
const UPDATE_FREQUENCY: usize = 60;

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
struct FrameCount {
    pub frame: u32,
}

pub struct NetPlugin;

impl Plugin for NetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        setup_ggrs(app);

        app.add_system(
            setup_socket
                .run_if(game_stage_is_online_lobby_menu)
                .run_if(there_is_no_socket_yet)
                .run_if(there_is_no_session_yet),
        )
        .add_system(
            setup_session
                .run_if(game_stage_is_online_lobby_menu)
                .run_if(there_is_a_socket)
                .run_if(there_is_no_session_yet),
        );
    }
}

fn setup_ggrs(mut app: &mut App) {
    app.insert_resource(FrameCount { frame: 0 });

    GGRSPlugin::<GGRSConfig>::new()
        .with_update_frequency(UPDATE_FREQUENCY)
        .with_input_system(ggrs_input_system)
        .register_rollback_type::<FrameCount>()
        .with_rollback_schedule(Schedule::default().with_stage(
            ROLLBACK_DEFAULT,
            SystemStage::parallel().with_system(increase_frame_system),
        ))
        .build(&mut app);
}

fn increase_frame_system(mut frame_count: ResMut<FrameCount>) {
    frame_count.frame += 1;
}

fn ggrs_input_system(
    _handle: In<PlayerHandle>,
    keyboard_input: Res<Input<KeyCode>>,
) -> BoxInput {
    let mut input: u8 = 0;

    BoxInput { inp: input }
}

fn setup_socket(mut commands: Commands) {
    info!("Connecting to matchbox server: {:?}", ROOM_URL);

    let (socket, message_loop) = WebRtcSocket::new(ROOM_URL);

    IoTaskPool::get().spawn(message_loop).detach();

    commands.insert_resource(Some(socket));
}

pub fn setup_session(
    mut commands: Commands,
    socket: Option<ResMut<Option<WebRtcSocket>>>,
) {
    let mut socket = socket.unwrap();
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
        .with_fps(UPDATE_FREQUENCY)
        .expect("Invalid FPS")
        .with_input_delay(2);

    for (i, player_type) in socket.players().iter().enumerate() {
        session_builder = session_builder
            .add_player(player_type.clone(), i)
            .expect("Invalid player added.");
    }

    // start the GGRS session
    let session = session_builder.start_p2p_session(socket).unwrap();

    commands.insert_resource(session);
    commands.insert_resource(SessionType::P2PSession);
}

fn game_stage_is_online_lobby_menu(game_state: Res<GameState>) -> bool {
    if game_state.game_stage == GameStage::OnlineLobbyMenu {
        return true;
    } else {
        return false;
    }
}

fn there_is_a_socket(socket: Option<Res<Option<WebRtcSocket>>>) -> bool {
    socket.is_some()
}

fn there_is_no_socket_yet(socket: Option<Res<Option<WebRtcSocket>>>) -> bool {
    socket.is_none()
}

fn there_is_no_session_yet(
    session: Option<ResMut<P2PSession<GGRSConfig>>>,
) -> bool {
    session.is_none()
}
