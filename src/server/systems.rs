use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_renet::renet::transport::{NetcodeServerTransport, NetcodeTransportError};
use bevy_renet::renet::{DefaultChannel, RenetServer, ServerEvent};

use super::{
    ClientConnection,
    DoKickPlayer,
    DoSendPacketToClient,
    OnClientConnected,
    OnClientDisconnected,
    OnReceivePacketFromClient,
};
use crate::common::{KickPacket, LoginData, PacketContainer};

pub(super) fn server_event_handler(
    transport: Res<NetcodeServerTransport>,
    mut clients: Query<(Entity, &mut ClientConnection)>,
    mut server_events: EventReader<ServerEvent>,
    mut connected_events: EventWriter<OnClientConnected>,
    mut disconnected_events: EventWriter<OnClientDisconnected>,
    mut commands: Commands,
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                let id = commands.spawn(ClientConnection::new(*client_id)).id();
                let login_data = transport
                    .user_data(*client_id)
                    .as_ref()
                    .map(|data| LoginData::from_bytes(data).unwrap());

                connected_events.send(OnClientConnected {
                    client_id: *client_id,
                    entity: id,
                    login_data,
                });

                info!("Client {} connected.", client_id);
            }
            ServerEvent::ClientDisconnected {
                client_id,
                reason: _,
            } => {
                let id = clients
                    .iter()
                    .find(|(_, connection)| connection.client_id() == *client_id)
                    .map(|(id, _)| id)
                    .unwrap();

                clients.get_mut(id).unwrap().1.disconnect();
                disconnected_events.send(OnClientDisconnected {
                    client_id: *client_id,
                    entity: id,
                });

                info!("Client {} disconnected.", client_id);
            }
        };
    }
}

pub(super) fn send_packet(
    mut server: ResMut<RenetServer>,
    mut events: EventReader<DoSendPacketToClient>,
) {
    for ev in events.read() {
        if !server.is_connected(ev.client_id) {
            continue;
        }

        let Some(message) = ev.packet.serialize() else {
            warn!("Failed to serialize packet!");
            continue;
        };

        server.send_message(ev.client_id, DefaultChannel::ReliableOrdered, message);
    }
}

pub(super) fn receive_packets(
    mut server: ResMut<RenetServer>,
    mut events: EventWriter<OnReceivePacketFromClient>,
) {
    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            let Some(packet) = PacketContainer::deserialize(&message) else {
                warn!("Failed to deserialize packet from {}!", client_id);
                continue;
            };

            events.send(OnReceivePacketFromClient { packet, client_id });
        }
    }
}

pub(super) fn error_handling(mut renet_error: EventReader<NetcodeTransportError>) {
    for e in renet_error.read() {
        error!("Networking Error: {}", e);
    }
}

pub(super) fn close_connections_on_exit(
    mut app_exit_evs: EventReader<AppExit>,
    mut server: ResMut<RenetServer>,
    mut transport: ResMut<NetcodeServerTransport>,
) {
    if app_exit_evs.read().next().is_none() {
        return;
    }
    server.disconnect_all();
    transport.disconnect_all(&mut server);
}

pub(super) fn kick_player(
    mut server: ResMut<RenetServer>,
    mut do_kick_players: EventReader<DoKickPlayer>,
    mut do_send_packet: EventWriter<DoSendPacketToClient>,
) {
    for ev in do_kick_players.read() {
        do_send_packet.send(DoSendPacketToClient {
            packet: KickPacket {
                reason: ev.reason.clone(),
            }
            .into(),
            client_id: ev.client_id,
        });
        server.disconnect(ev.client_id);
    }
}
