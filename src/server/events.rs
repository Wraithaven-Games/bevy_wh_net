use bevy::prelude::*;
use bevy_renet::renet::ClientId;

use crate::common::{LoginData, PacketContainer};

#[derive(Debug, Event)]
pub struct OnClientConnected {
    pub client_id: ClientId,
    pub entity: Entity,
    pub login_data: Option<LoginData>,
}

#[derive(Debug, Event)]
pub struct OnClientDisconnected {
    pub client_id: ClientId,
    pub entity: Entity,
}

#[derive(Debug, Event)]
pub struct DoSendPacketToClient {
    pub packet: PacketContainer,
    pub client_id: ClientId,
}

#[derive(Debug, Event)]
pub struct OnReceivePacketFromClient {
    pub packet: PacketContainer,
    pub client_id: ClientId,
}

#[derive(Debug, Event)]
pub struct DoKickPlayer {
    pub client_id: ClientId,
    pub reason: String,
}
