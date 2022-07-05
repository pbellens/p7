use std::net::SocketAddr;
use orderbook::commands;
use std::fmt::Display;

pub mod handler;

#[derive(Debug)]
pub enum IoEvent {
    Connect(SocketAddr), 
    ConnectCheck, 
    Disconnect,
    Req(commands::Cmd)
}

#[derive(Debug)]
pub enum IoReply {
    Stum,
    Reply(String)
}

impl Display for IoReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IoReply::Stum => write!(f, ""),
            IoReply::Reply(r) => write!(f, "{}", r),
        }
    }
}
