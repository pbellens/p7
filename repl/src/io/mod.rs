use std::net::SocketAddr;
use orderbook::commands;

pub mod handler;

#[derive(Debug, Clone)]
pub enum IoEvent {
    Connect(SocketAddr), 
    ConnectCheck, 
    Disconnect,
    Buy(commands::Cmd)
}
