extern crate orderbook;
use orderbook::orderbook as ob;
use orderbook::commands;
use orderbook::snapshot;
use futures::prelude::*;
use tokio::net::TcpListener;
use serde_json::Value;
use std::sync::{Arc, Mutex};
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};
use tokio_serde::formats::SymmetricalJson;


#[tokio::main]
async fn main() 
{
    let listener = TcpListener::bind("127.0.0.1:17653").await.unwrap();
    println!("🚀 listening on {:?}", listener.local_addr());
    let motherbook = Arc::new(Mutex::new(ob::OrderBook::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        println!("got new connection!");
        let (read, write) = socket.into_split();

        let book = Arc::clone(&motherbook);

        let length_delimited_read = FramedRead::new(read, LengthDelimitedCodec::new());
        let mut deserialized = tokio_serde::SymmetricallyFramed::new(length_delimited_read, SymmetricalJson::<Value>::default()); 
        let length_delimited_write = FramedWrite::new(write, LengthDelimitedCodec::new());
        let mut serialized = tokio_serde::SymmetricallyFramed::new(length_delimited_write, SymmetricalJson::<Value>::default()); 

         tokio::spawn(async move {
            while let Some(msg) = deserialized.try_next().await.unwrap() 
            {
                //let order: orders::Order = serde_json::from_value(msg).unwrap();
                println!("got msg {:?}", msg);
                match serde_json::from_value(msg) {
                    Ok(cmd) => match cmd {
                        commands::Cmd::Order(o) => 
                        { 
                            let mut ob = book.lock().unwrap();
                            println!("got order {:?}", o);
                            ob.execute(o); 
                        },
                        commands::Cmd::Snapshot(depth) => 
                        {
                            println!("got snapshot request for {}", depth);
                            let json = 
                            {
                                let ob = book.lock().unwrap();
                                serde_json::to_value(snapshot::Snapshot::new(&*ob, depth)).unwrap()
                            };
                            serialized.send(json).await.unwrap();
                        }
                    },
                    Err(e) => println!("{}", e)
                }
            }
        });
    }
}


