use futures::TryStreamExt;
use futures::SinkExt;
use orderbook::commands;
use tokio::net::TcpStream;
use tokio_serde::formats::SymmetricalJson;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};
use std::net::SocketAddr;
use serde_json::Value;
use super::IoEvent;

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
        // Delimit frames using a length header
        //let length_delimited = FramedWrite::new(socket, LengthDelimitedCodec::new());
        // Serialize frames with JSON
        //let mut serialized =
        //    tokio_serde::SymmetricallyFramed::new(length_delimited, SymmetricalJson::default());

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
                                println!("got answer {:?}", msg);
                            } 
                        }
                    }
                }
            },
        };

            //let Some(msg) = deserialized.try_next().await.unwrap() 
            //{
            //    //let order: orders::Order = serde_json::from_value(msg).unwrap();
            //    println!("got msg {:?}", msg);
            //    match serde_json::from_value(msg) {
            //        Ok(cmd) => match cmd {
            //            commands::Cmd::Order(o) => 
            //            { 
            //                println!("got order {:?}", o);
            //                book.execute(o); 
            //            },
            //            commands::Cmd::Snapshot(depth) => 
            //            {
            //                println!("got snapshot request for {}", depth);
            //                let s  = serde_json::to_value(snapshot::Snapshot::new(&book, depth)).unwrap();
            //                serialized.send(s).await.unwrap();
            //            }
            //        },
            //        Err(e) => println!("{}", e)
            //    }
            //}
 

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

    ///// Just take a little break
    //async fn do_sleep(&mut self, duration: Duration) -> Result<()> {
    //    info!("😴 Go sleeping for {:?}...", duration);
    //    tokio::time::sleep(duration).await;
    //    info!("⏰ Wake up !");
    //    // Notify the app for having slept
    //    let mut app = self.app.lock().await;
    //    app.sleeped();

    //    Ok(())
    //}
}
