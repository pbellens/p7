use std::time::Duration;
use std::net::SocketAddr;
use orderbook::data::*;

pub mod handler;

#[derive(Debug, Clone)]
pub enum IoEvent {
    Connect(SocketAddr), 
    ConnectCheck, 
    Disconnect,
    Buy(orders::Order)
}
