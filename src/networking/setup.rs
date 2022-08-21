use bevy::{core::Pod, prelude::*, tasks::*};
use bevy_ggrs::GGRSPlugin;
use bytemuck::Zeroable;
use ggrs::Config;
use matchbox_socket::WebRtcNonBlockingSocket;

#[repr(C)]
#[derive(Copy, PartialEq, Clone, Pod, Zeroable)]
struct Test {
    a: u16,
    b: u16,
}

#[derive(Debug)]
pub struct GGRSConfig;
impl Config for GGRSConfig {
    type Input = Test;
    type State = u8;
    type Address = String;
}

pub struct NetworkingPlugin;

fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://192.168.2.170:3536/next_2";

    info!("connecting to matchbox server: {:?}", room_url);

    let (socket, message_loop) = WebRtcNonBlockingSocket::new(room_url);

    // The message loop needs to be awaited, or nothing will happen.
    // We do this here using bevy's task system.
    IoTaskPool::get().spawn(message_loop).detach();

    commands.insert_resource(Some(socket));
}

fn wait_for_players(
    mut commands: Commands,
    mut socket: ResMut<Option<WebRtcNonBlockingSocket>>,
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
    let mut p2p_session = ggrs::P2PSession

    // ggrs::P2PSession::new_with_socket(
    //     num_players as u32,
    //     INPUT_SIZE,
    //     max_prediction,
    //     socket,
    // );

    for (i, player) in players.into_iter().enumerate() {
        p2p_session
            .add_player(player, i)
            .expect("failed to add player");

        if player == PlayerType::Local {
            // set input delay for the local player
            p2p_session.set_frame_delay(2, i).unwrap();
        }
    }

    // start the GGRS session
    commands.start_p2p_session(p2p_session);

    // TODO
}

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(start_matchbox_socket)
            .add_system(wait_for_players);

        GGRSPlugin::new().build(app);
    }
}
