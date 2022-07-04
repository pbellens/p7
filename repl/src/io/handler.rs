use futures::TryStreamExt;
use futures::SinkExt;
use orderbook::commands;
use tokio::net::TcpStream;
use tokio_serde::formats::SymmetricalJson;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};
use std::net::SocketAddr;
use serde_json::Value;
use super::IoEvent;
use orderbook::snapshot::Snapshot;

//use log::{error, info};
//use crate::app::App;

/// In the IO thread, we handle IO event without blocking the REPL thread
pub struct AsyncHandler {
    pub addr: Option<SocketAddr>,
    pub stream: Option<TcpStream>,
}

impl AsyncHandler {
    pub fn new(saddr: SocketAddr) {
    }

    /// We could be async here
    pub async fn handle_io_event(&mut self, io_event: IoEvent) {
        let result = match io_event {
            IoEvent::Connect(addr) => {
                match TcpStream::connect(addr).await
                {
                    Ok(s) => {
                        self.addr = Some(addr);
                        self.stream = Some(s);
                    },
                    Err(e) => println!("Could not connect: {}", e) 
                };
            },
            IoEvent::ConnectCheck => {
                match self.stream {
                    None => println!("Not connected."),
                    Some(_) => println!("Connected to {:?}.", self.addr),
                }
            },
            IoEvent::Disconnect => {
                self.stream = None;
            },
            IoEvent::Req(cmd) => {
                match &mut self.stream {
                    None => println!("Not connected, can t submit."),
                    Some(str) => {
                        let (read, write) = str.split();

                        let length_delimited_write = FramedWrite::new(write, LengthDelimitedCodec::new());
                        let mut serialized =
                            tokio_serde::SymmetricallyFramed::new(length_delimited_write, SymmetricalJson::<Value>::default());
                        serialized
                            .send(serde_json::to_value(cmd).unwrap())
                            .await
                            .unwrap();

                        if let commands::Cmd::Snapshot(depth) = cmd {
                            let length_delimited_read = FramedRead::new(read, LengthDelimitedCodec::new());
                            let mut deserialized = tokio_serde::SymmetricallyFramed::new(length_delimited_read, SymmetricalJson::<Value>::default()); 
                            if let Some(msg) = deserialized.try_next().await.unwrap() {
                                let snapshot: Snapshot = serde_json::from_value(msg).unwrap();
                                let s = format!("{}", snapshot);
                                Ok(s)
                            } 
                        }
                    }
                }
            },
            IoEvent::Reply(_) => todo!(),
        };

        //if let Err(err) = result {
        //    error!("Oops, something wrong happen: {:?}", err);
        //}

        //let mut app = self.app.lock().await;
        //app.loaded();
    }

    ///// We use dummy implementation here, just wait 1s
    async fn do_connect(&mut self, addr: SocketAddr) {
        self.stream = Some(TcpStream::connect(addr).await.unwrap());
    }
}

