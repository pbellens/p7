use futures::TryStreamExt;
use futures::SinkExt;
use orderbook::commands;
use tokio::net::TcpStream;
use tokio_serde::formats::SymmetricalJson;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};
use std::net::SocketAddr;
use anyhow::{Result, Ok};
use serde_json::Value;
use super::{IoEvent, IoReply};
use orderbook::snapshot::Snapshot;

//use log::{error, info};
//use crate::app::App;

/// In the IO thread, we handle IO event without blocking the REPL thread
pub struct AsyncHandler {
    pub addr: Option<SocketAddr>,
    pub stream: Option<TcpStream>,
}

impl AsyncHandler {
    /// We could be async here
    pub async fn handle_io_event(&mut self, io_event: IoEvent) -> Result<IoReply> {
        match io_event {
            IoEvent::Connect(addr) => {
                let stream =  TcpStream::connect(addr).await?;
                self.addr = Some(addr);
                self.stream = Some(stream);
                Ok(IoReply::Stum)
            },
            IoEvent::ConnectCheck => {
                match self.stream {
                    None => Ok(IoReply::Reply("Not connected.".to_string())),
                    Some(_) => Ok(IoReply::Reply(format!("Connected to {:?}.", self.addr)))
                }
            },
            IoEvent::Disconnect => {
                self.stream = None;
                Ok(IoReply::Stum)
            },
            IoEvent::Req(cmd) => {
                match &mut self.stream {
                    None => Ok(IoReply::Reply("Not connected, can t submit.".to_string())),
                    Some(str) => {
                        let (read, write) = str.split();

                        let length_delimited_write = FramedWrite::new(write, LengthDelimitedCodec::new());
                        let mut serialized =
                            tokio_serde::SymmetricallyFramed::new(length_delimited_write, SymmetricalJson::<Value>::default());
                        serialized
                            .send(serde_json::to_value(cmd).unwrap())
                            .await?;

                        match cmd {
                            commands::Cmd::Snapshot(_depth)  => {
                                let length_delimited_read = FramedRead::new(read, LengthDelimitedCodec::new());
                                let mut deserialized = tokio_serde::SymmetricallyFramed::new(length_delimited_read, SymmetricalJson::<Value>::default()); 
                                let msg = deserialized.try_next().await?;
                                let snapshot: Snapshot = serde_json::from_value(msg.unwrap())?;
                                let s = format!("{}", snapshot);
                                Ok(IoReply::Reply(s))
                            },
                            commands::Cmd::Order(_) => Ok(IoReply::Stum)
                        }
                    }
                }
            },
        }

        //if let Err(err) = result {
        //    error!("Oops, something wrong happen: {:?}", err);
        //}

        //let mut app = self.app.lock().await;
        //app.loaded();
    }
}
