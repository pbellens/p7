use std::time::Duration;
use std::net::SocketAddr;

pub mod handler;

#[derive(Debug, Clone)]
pub enum IoEvent {
    Initialize,      
    Sleep(Duration), 
    Connect(SocketAddr), 
}

