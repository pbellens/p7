extern crate orderbook;
use orderbook::commands;
use orderbook::data::{orders, side};
use easy_repl::{Repl, CommandStatus, command};
use anyhow::{self, Context};
use crossbeam_channel::unbounded;
mod io;
use io::handler::AsyncHandler;
use std::net::SocketAddr;
use crate::io::IoEvent;

#[tokio::main]
async fn main() -> anyhow::Result<()> 
{
    let (reqs, reqr) = unbounded::<IoEvent>();
    let (rpls, rplr) = unbounded::<IoEvent>();

     tokio::spawn(async move {
        let mut handler = AsyncHandler{ addr: None, stream: None };
        while let Ok(io_event) = reqr.recv() {
            let repl = handler.handle_io_event(io_event).await;
            rpls.send(repl);
        }
    });

    let sconnect = reqs.clone();
    let scheck = reqs.clone();
    let sdisconnect = reqs.clone();
    let sbuy = reqs.clone();
    let ssnapshot = reqs.clone();
    let rsnapshot = rplr.clone();
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
       .add("limit-buy", command! {
           "Place a buy limit order",
           (prod:u32, qty:u64, price:u64) => |prod, qty, pr| {
               let buy = commands::Cmd::Order(orders::Order{ prod: prod, qty: qty, price: pr, side: side::Side::Buy, kind: orders::OrderType::LimitOrder });
               sbuy.send(IoEvent::Req(buy)).unwrap();
               Ok(CommandStatus::Done)
           }
        })
        .add("snapshot", command! {
           "Create a snapshot of the orderbook",
           (prod: u32, depth:usize) => |_prod, depth| {
               let req = commands::Cmd::Snapshot(depth);
               ssnapshot.send(IoEvent::Req(req)).unwrap();
               let dum = rsnapshot.recv().unwrap();
               println!("got repl {:?}", dum);
               Ok(CommandStatus::Done)
           }
       })
       .build().context("Failed to create REPL")?;

    repl.run().context("Critical REPL error")?;

    Ok(())
}

