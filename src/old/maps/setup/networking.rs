use bevy::{core::Pod, input::*, prelude::*, tasks::*};
use bevy_ggrs::{GGRSPlugin, SessionType};
use bytemuck::Zeroable;
use ggrs::{Config, PlayerHandle, PlayerType, SessionBuilder};
use matchbox_socket::WebRtcSocket;

use crate::resources::game_state::GameState;

const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct BoxInput {
    pub inp: u8,
}

pub struct LocalHandles {
    pub handles: Vec<PlayerHandle>,
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

pub struct NetworkingPlugin;

pub fn network_input_system(
    _handle: In<PlayerHandle>,
    keyboard_input: Res<Input<KeyCode>>,
) -> BoxInput {
    let mut input: u8 = 0;

    if keyboard_input.pressed(KeyCode::W) {
        input |= INPUT_UP;
    }
    if keyboard_input.pressed(KeyCode::A) {
        input |= INPUT_LEFT;
    }
    if keyboard_input.pressed(KeyCode::S) {
        input |= INPUT_DOWN;
    }
    if keyboard_input.pressed(KeyCode::D) {
        input |= INPUT_RIGHT;
    }

    BoxInput { inp: input }
}

fn create_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://192.168.2.170:3536/next_2";

    info!("connecting to matchbox server: {:?}", room_url);

    let (socket, message_loop) = WebRtcSocket::new(room_url);

    // The message loop needs to be awaited, or nothing will happen.
    // We do this here using bevy's task system.
    IoTaskPool::get().spawn(message_loop).detach();

    commands.insert_resource(Some(socket));
}

fn create_ggrs_sessions(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut socket: ResMut<Option<WebRtcSocket>>,
) {
    if game_state.started {
        return; // Game already started, no need to create sessions
    }

    let socket = socket.as_mut();

    // If there is no socket we've already started the game
    if socket.is_none() {
        info!("No socket");
        return;
    }

    // Check for new connections
    socket.as_mut().unwrap().accept_new_connections();
    let players = socket.as_ref().unwrap().players();

    let num_players = 2;
    if players.len() < num_players {
        return; // wait for more players
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
    let session = session_builder.start_p2p_session(socket);

    for a in session.iter() {
        println!("{:?}", a.num_players());
    }

    commands.insert_resource(session);
    commands.insert_resource(LocalHandles { handles });
    commands.insert_resource(SessionType::P2PSession);

    game_state.started = true;
}

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_matchbox_socket)
            .add_system(create_ggrs_sessions);

        GGRSPlugin::<GGRSConfig>::new()
            .with_input_system(network_input_system)
            .build(app);
    }
}
