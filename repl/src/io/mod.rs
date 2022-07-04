use std::net::SocketAddr;
use orderbook::commands;

pub mod handler;

#[derive(Debug)]
pub enum IoEvent {
    Reply(anyhow::Result<String>),
    Connect(SocketAddr), 
    ConnectCheck, 
    Disconnect,
    Req(commands::Cmd)
}
