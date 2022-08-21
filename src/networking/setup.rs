use bevy::{core::Pod, prelude::*, tasks::*};
use bevy_ggrs::{GGRSPlugin, SessionType};
use bytemuck::Zeroable;
use ggrs::{Config, PlayerHandle, PlayerType, SessionBuilder};
use matchbox_socket::WebRtcSocket;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct Input {
    pub inp: u8,
}

pub struct LocalHandles {
    pub handles: Vec<PlayerHandle>,
}

#[derive(Debug)]
pub struct GGRSConfig;
impl Config for GGRSConfig {
    type Input = Input;
    type State = u8;
    type Address = String;
}

pub struct NetworkingPlugin;

fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://192.168.2.170:3536/next_2";

    info!("connecting to matchbox server: {:?}", room_url);

    let (socket, message_loop) = WebRtcSocket::new(room_url);

    // The message loop needs to be awaited, or nothing will happen.
    // We do this here using bevy's task system.
    IoTaskPool::get().spawn(message_loop).detach();

    commands.insert_resource(Some(socket));
}

fn wait_for_players(
    mut commands: Commands,
    mut socket: ResMut<Option<WebRtcSocket>>,
) {
    let socket = socket.as_mut();

    // If there is no socket we've already started the game
    if socket.is_none() {
        info!("No socket");
        return;
    }

    // Check for new connections
    socket.as_mut().unwrap().accept_new_connections();
    let players = socket.as_ref().unwrap().players();

    let connected_peers = socket.as_ref().unwrap().connected_peers();

    info!("{:?}", connected_peers);

    let num_players = 2;
    if players.len() < num_players {
        return; // wait for more players
    }

    info!("All peers have joined, going in-game");
    // consume the socket (currently required because GGRS takes ownership of its socket)
    let socket = socket.take().unwrap();

    let max_prediction = 12;

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

    commands.insert_resource(session);
    commands.insert_resource(LocalHandles { handles });
    commands.insert_resource(SessionType::P2PSession);
}

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(start_matchbox_socket)
            .add_system(wait_for_players);

        GGRSPlugin::<GGRSConfig>::new().build(app);
    }
}
