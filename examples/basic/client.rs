use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy_wh_net::client::{ClientNetworkingPlugin, DoConnectToServer};
use bevy_wh_net::common::LoginData;

pub fn run() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(LogPlugin {
            level: Level::DEBUG,
            ..default()
        })
        .add_plugins(ClientNetworkingPlugin)
        .add_systems(Startup, init)
        .run()
}

fn init(mut do_connect_to_server: EventWriter<DoConnectToServer>) {
    let username = "Player".into();
    let password = "Password".into();
    let login_data = LoginData::new(username, password).unwrap();

    do_connect_to_server.send(DoConnectToServer {
        ip: "127.0.0.1:8123".into(),
        login_data: Some(login_data),
    })
}
