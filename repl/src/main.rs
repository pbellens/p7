extern crate orderbook;
use orderbook::data::*;
use easy_repl::{Repl, CommandStatus, command};
use anyhow::{self, Context};
//use futures::prelude::*;
//use serde_json::json;
use tokio::net::TcpStream;
use tokio_serde::formats::*;
use tokio_util::codec::{FramedWrite, LengthDelimitedCodec};
use crossbeam_channel::unbounded;
mod io;
use io::handler::AsyncHandler;
use std::net::SocketAddr;
use crate::io::IoEvent;

//
//    // Delimit frames using a length header
//    let length_delimited = FramedWrite::new(socket, LengthDelimitedCodec::new());
//
//    // Serialize frames with JSON
//    let mut serialized =
//        tokio_serde::SymmetricallyFramed::new(length_delimited, SymmetricalJson::default());
//
//    let buy = orders::Order{ prod: 2, qty: 10, price: 2, side: side::Side::Buy, kind: orders::OrderType::LimitOrder };
//    let sell = orders::Order{ prod: 2, qty: 30, price: 4, side: side::Side::Sell, kind: orders::OrderType::LimitOrder };
//    // Send the value
//    serialized
//        .send(serde_json::to_value(buy).unwrap())
//        .await
//        .unwrap();
//    serialized
//        .send(serde_json::to_value(sell).unwrap())
//        .await
//        .unwrap();
//}



#[tokio::main]
async fn main() -> anyhow::Result<()> 
{
    let (s, r) = unbounded::<IoEvent>();

     tokio::spawn(async move {
        let mut handler = AsyncHandler{ stream: None };
        while let Ok(io_event) = r.recv() {
            handler.handle_io_event(io_event).await;
        }
    });

    // Bind a server socket
    //let socket = TcpStream::connect("127.0.0.1:17653").await.unwrap();
    // Delimit frames using a length header
    //let length_delimited = FramedWrite::new(socket, LengthDelimitedCodec::new());
    // Serialize frames with JSON
    //let mut serialized =
    //    tokio_serde::SymmetricallyFramed::new(length_delimited, SymmetricalJson::default());

     let mut repl = Repl::builder()
        .add("connect", command! {
            "Connect to P7 instance.",
            (addr: SocketAddr) => |addr: SocketAddr| {
                println!("Got address {}", addr);
                s.send(IoEvent::Connect(addr)).unwrap();
                Ok(CommandStatus::Done)
            }
        })
        .add("buy", command! {
            "Buy",
            (prod:u32, qty:u64, price:u64) => |prod, qty, pr| {
                let buy = orders::Order{ prod: prod, qty: qty, price: pr, side: side::Side::Buy, kind: orders::OrderType::LimitOrder };
                //serialized
                //    .send(serde_json::to_value(buy).unwrap())
                //    .await
                //    .unwrap();
                Ok(CommandStatus::Done)
            }
        })
        .build().context("Failed to create REPL")?;

    repl.run().context("Critical REPL error")?;



    Ok(())
}
