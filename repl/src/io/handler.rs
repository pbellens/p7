use std::time::Duration;
use futures::SinkExt;
use tokio::net::TcpStream;
use tokio_serde::formats::SymmetricalJson;
use tokio_util::codec::{FramedWrite, LengthDelimitedCodec};
use std::net::SocketAddr;
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
            IoEvent::Buy(order) => {
                match &mut self.stream {
                    None => println!("Not connected, can t place orders."),
                    Some(str) => {
                        let length_delimited = FramedWrite::new(str, LengthDelimitedCodec::new());
                        let mut serialized =
                            tokio_serde::SymmetricallyFramed::new(length_delimited, SymmetricalJson::default());
                        serialized
                            .send(serde_json::to_value(order).unwrap())
                            .await
                            .unwrap();
                    }
                }
            },
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
