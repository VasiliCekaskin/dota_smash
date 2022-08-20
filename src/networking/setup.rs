use bevy::{prelude::*, tasks::*};
use matchbox_socket::WebRtcNonBlockingSocket;

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

fn wait_for_players(mut socket: ResMut<Option<WebRtcNonBlockingSocket>>) {
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
    // TODO
}

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(start_matchbox_socket)
            .add_system(wait_for_players);
    }
}
