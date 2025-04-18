use bevy::{
    prelude::{
        AssetServer, Camera3d, Commands, Entity, EventReader, EventWriter, Query, Res, ResMut, With,
    },
    window::{CursorGrabMode, PrimaryWindow, Window},
};
use bevy_egui::{egui, EguiContexts};

use rose_data::ZoneId;
use rose_game_common::messages::client::ClientMessage;

use crate::{
    animation::CameraAnimation,
    events::{LoadZoneEvent, LoginEvent, NetworkEvent},
    resources::{Account, LoginConnection, LoginState, ServerConfiguration, ServerList},
    systems::{FreeCamera, OrbitCamera},
};

pub fn login_state_enter_system(
    mut commands: Commands,
    mut loaded_zone: EventWriter<LoadZoneEvent>,
    mut query_window: Query<&mut Window, With<PrimaryWindow>>,
    query_cameras: Query<Entity, With<Camera3d>>,
    asset_server: Res<AssetServer>,
) {
    // Ensure cursor is not locked
    if let Ok(mut window) = query_window.get_single_mut() {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }

    // Reset camera
    for entity in query_cameras.iter() {
        commands
            .entity(entity)
            .remove::<FreeCamera>()
            .remove::<OrbitCamera>()
            .insert(CameraAnimation::repeat(
                //asset_server.load("3DDATA/TITLE/CAMERA01_INTRO01.ZMO"), //4
                asset_server.load("3DDATA/TITLE/CAMERA01_INTRON.ZMO"), //7
                None,
            ));
    }

    commands.remove_resource::<Account>();
    commands.insert_resource(LoginState::Input);

    loaded_zone.send(LoadZoneEvent::new(ZoneId::new(7).unwrap())); // @TODO make a array with random maps to choose from and right cameras
}

pub fn login_state_exit_system(mut commands: Commands) {
    commands.remove_resource::<LoginConnection>();
    commands.remove_resource::<LoginState>();
}

pub fn login_system(
    mut egui_context: EguiContexts,
    login_connection: Option<Res<LoginConnection>>,
    mut login_state: ResMut<LoginState>,
    server_list: Option<Res<ServerList>>,
) {
    if !matches!(*login_state, LoginState::Input) && login_connection.is_none() {
        // When we lose login server connection, return to login
        *login_state = LoginState::Input;
    }

    if matches!(*login_state, LoginState::WaitServerList) && server_list.is_some() {
        // We have server list, transition to select
        *login_state = LoginState::ServerSelect;
    }

    match *login_state {
        LoginState::WaitServerList => {
            egui::Window::new("Connecting...")
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .collapsible(false)
                .show(egui_context.ctx_mut(), |ui| {
                    ui.label("Logging in");
                });
        }
        LoginState::JoiningServer => {
            egui::Window::new("Connecting...")
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .collapsible(false)
                .show(egui_context.ctx_mut(), |ui| {
                    ui.label("Connecting to channel");
                });
        }
        _ => {}
    }
}

pub fn login_event_system(
    mut commands: Commands,
    mut login_state: ResMut<LoginState>,
    mut login_events: EventReader<LoginEvent>,
    login_connection: Option<Res<LoginConnection>>,
    server_configuration: Res<ServerConfiguration>,
    mut network_events: EventWriter<NetworkEvent>,
) {
    for event in login_events.iter() {
        match event {
            LoginEvent::Login { username, password } => {
                if matches!(*login_state, LoginState::Input) {
                    *login_state = LoginState::WaitServerList;

                    commands.insert_resource(Account {
                        username: username.clone(),
                        password: password.clone(),
                    });

                    network_events.send(NetworkEvent::ConnectLogin {
                        ip: server_configuration.ip.clone(),
                        port: server_configuration.port.parse::<u16>().unwrap_or(29000),
                    });
                }
            }
            &LoginEvent::SelectServer {
                server_id,
                channel_id,
            } => {
                if let Some(login_connection) = &login_connection {
                    login_connection
                        .client_message_tx
                        .send(ClientMessage::JoinServer {
                            server_id,
                            channel_id,
                        })
                        .ok();
                }
                *login_state = LoginState::JoiningServer;
            }
        }
    }
}
