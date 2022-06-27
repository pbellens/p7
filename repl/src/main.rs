extern crate orderbook;
use orderbook::data::*;
use easy_repl::{Repl, CommandStatus, command};
use anyhow::{self, Context};
//use futures::prelude::*;
//use serde_json::json;
use crossbeam_channel::unbounded;
mod io;
use io::handler::AsyncHandler;
use std::net::SocketAddr;
use crate::io::IoEvent;

#[tokio::main]
async fn main() -> anyhow::Result<()> 
{
    let (s, r) = unbounded::<IoEvent>();

     tokio::spawn(async move {
        let mut handler = AsyncHandler{ addr: None, stream: None };
        while let Ok(io_event) = r.recv() {
            handler.handle_io_event(io_event).await;
        }
    });

    let sconnect = s.clone();
    let scheck = s.clone();
    let sdisconnect = s.clone();
    let sbuy = s.clone();
    let mut repl = Repl::builder()
       .add("connect", command! {
           "Connect to P7 instance.",
           (addr: SocketAddr) => |addr: SocketAddr| {
               sconnect.send(IoEvent::Connect(addr)).unwrap();
               Ok(CommandStatus::Done)
           }
       })
       .add("ping", command! {
           "Check connection to P7 instance.",
           () => || {
               scheck.send(IoEvent::ConnectCheck).unwrap();
               Ok(CommandStatus::Done)
           }
       })
       .add("disconnect", command! {
           "Disconnect from P7 instance.",
           () => || {
               sdisconnect.send(IoEvent::Disconnect).unwrap();
               Ok(CommandStatus::Done)
           }
       })
       .add("buy-limit", command! {
           "Place a buy limit order",
           (prod:u32, qty:u64, price:u64) => |prod, qty, pr| {
               let buy = orders::Order{ prod: prod, qty: qty, price: pr, side: side::Side::Buy, kind: orders::OrderType::LimitOrder };
               sbuy.send(IoEvent::Buy(buy)).unwrap();
               Ok(CommandStatus::Done)
           }
       })
       .build().context("Failed to create REPL")?;

    repl.run().context("Critical REPL error")?;

    Ok(())
}
