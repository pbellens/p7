extern crate orderbook;
use orderbook::orderbook as ob;
use orderbook::data::orders;
use futures::prelude::*;
use tokio::net::TcpListener;
use serde_json::Value;
use tokio_util::codec::{FramedRead, LengthDelimitedCodec};
use tokio_serde::formats::SymmetricalJson;



#[tokio::main]
async fn main() 
{
    let listener = TcpListener::bind("127.0.0.1:17653").await.unwrap();
    println!("Listening on {:?}", listener.local_addr());

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        let length_delimited = FramedRead::new(socket, LengthDelimitedCodec::new());
        let mut deserialized = tokio_serde::SymmetricallyFramed::new(length_delimited, SymmetricalJson::<Value>::default()); 

         tokio::spawn(async move {
            let mut book = ob::OrderBook::new();
            while let Some(msg) = deserialized.try_next().await.unwrap() {
                let order: orders::Order = serde_json::from_value(msg).unwrap();
                println!("got order {}", order);
                book.execute(order);
            }
        });
    }
}
