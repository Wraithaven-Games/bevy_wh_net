use bevy::prelude::*;

use crate::common::{LoginData, PacketContainer};

#[derive(Debug, Event)]
pub struct OnConnectToServer;

#[derive(Debug, Event)]
pub struct OnDisconnectFromServer;

#[derive(Debug, Event)]
pub struct OnCouldNotConnectToServer;

#[derive(Debug, Event, Deref)]
pub struct DoSendPacketToServer(pub PacketContainer);

#[derive(Debug, Event, Deref)]
pub struct OnReceivePacketFromServer(pub PacketContainer);

#[derive(Debug, Event)]
pub struct DoConnectToServer {
    pub ip: String,
    pub login_data: Option<LoginData>,
}

#[derive(Debug, Event)]
pub struct DoDisconnectFromServer;
