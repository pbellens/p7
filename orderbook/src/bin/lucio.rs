extern crate orderbook;
use orderbook::orderbook as ob;
use orderbook::commands;
use futures::prelude::*;
use tokio::net::TcpListener;
use serde_json::Value;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};
use tokio_serde::formats::SymmetricalJson;


#[tokio::main]
async fn main() 
{
    let listener = TcpListener::bind("127.0.0.1:17653").await.unwrap();
    println!("Listening on {:?}", listener.local_addr());

    loop {
        println!("got new connection!");
        let (socket, _) = listener.accept().await.unwrap();
        let (read, write) = socket.into_split();

        let length_delimited_read = FramedRead::new(read, LengthDelimitedCodec::new());
        let mut deserialized = tokio_serde::SymmetricallyFramed::new(length_delimited_read, SymmetricalJson::<Value>::default()); 
        let length_delimited_write = FramedWrite::new(write, LengthDelimitedCodec::new());
        let mut serialized = tokio_serde::SymmetricallyFramed::new(length_delimited_write, SymmetricalJson::<Value>::default()); 

         tokio::spawn(async move {
            let mut book = ob::OrderBook::new();
            while let Some(msg) = deserialized.try_next().await.unwrap() 
            {
                //let order: orders::Order = serde_json::from_value(msg).unwrap();
                match serde_json::from_value(msg) {
                    Ok(cmd) => match cmd {
                        commands::Cmd::Order(o) => 
                        { 
                            book.execute(o); 
                        },
                        commands::Cmd::Snapshot(depth) => 
                        {
                            let s  = serde_json::to_value(snapshot::Snapshot::new(&book, 4)).unwrap();
                            serialized.send(s).await.unwrap();
                        }
                    },
                    Err(e) => println!("{}", e)
                }
            }
        });
    }
}
