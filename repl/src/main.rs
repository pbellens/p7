extern crate orderbook;
use orderbook::commands;
use orderbook::data::{orders, side};
use easy_repl::{Repl, CommandStatus, command};
use anyhow::{self, Context};
use crossbeam_channel::unbounded;
mod io;
use io::handler::AsyncHandler;
use std::net::SocketAddr;
use crate::io::{IoEvent, IoReply};

fn handle(r: &IoReply) {
    match r {
        IoReply::Reply(rpl) => println!("{}",rpl),
        IoReply::Stum => (), 
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> 
{
    let (reqs, reqr) = unbounded::<IoEvent>();
    let (rpls, rplr) = unbounded::<IoReply>();

     tokio::spawn(async move {
        let mut handler = AsyncHandler{ addr: None, stream: None };
        while let Ok(io_event) = reqr.recv() {
            let repl = handler.handle_io_event(io_event).await.unwrap();
            rpls.send(repl);
        }
    });

    let sconnect = reqs.clone();
    let rconnect = rplr.clone();
    let scheck = reqs.clone();
    let rcheck = rplr.clone();
    let sdisconnect = reqs.clone();
    let rdisconnect = rplr.clone();
    let sbuy = reqs.clone();
    let rbuy = rplr.clone();
    let ssnapshot = reqs.clone();
    let rsnapshot = rplr.clone();

    let mut repl = Repl::builder()
        .add("connect", command! {
            "Connect to P7 instance.",
            (addr: SocketAddr) => |addr: SocketAddr| {
                sconnect.send(IoEvent::Connect(addr)).unwrap();
                handle(&rconnect.recv().unwrap());
                Ok(CommandStatus::Done)
           }
        })
        .add("ping", command! {
            "Check connection to P7 instance.",
            () => || {
                scheck.send(IoEvent::ConnectCheck).unwrap();
                handle(&rcheck.recv().unwrap());
                Ok(CommandStatus::Done)
            }
        })
        .add("disconnect", command! {
            "Disconnect from P7 instance.",
            () => || {
                sdisconnect.send(IoEvent::Disconnect).unwrap();
                handle(&rdisconnect.recv().unwrap());
                Ok(CommandStatus::Done)
            }
        })
        .add("order", command! {
        "Place an order",
            (side: String, kind: String, prod:u32, qty:u64, price:u64) => |side: String, kind: String, prod, qty, pr| {
                let s = match side.as_str() {
                    "buy" => side::Side::Buy, 
                    "sell" => side::Side::Sell, 
                    _ => unreachable!()
                };
                let k = match kind.as_str() {
                    "limit" => orders::OrderType::LimitOrder,
                    "market" => orders::OrderType::MarketOrder,
                    _ => unreachable!()
                };
                let order = orders::Order{ prod: prod, qty: qty, price: pr, side: s, kind: k };
                let cmd = commands::Cmd::Order(order);
                sbuy.send(IoEvent::Req(cmd)).unwrap();
                handle(&rbuy.recv().unwrap());
                Ok(CommandStatus::Done)
            }
        })
        .add("snapshot", command! {
            "Create a snapshot of the orderbook",
            (prod: u32, depth:usize) => |_prod, depth| {
                let req = commands::Cmd::Snapshot(depth);
                ssnapshot.send(IoEvent::Req(req)).unwrap();
                match rsnapshot.recv().unwrap() {
                    IoReply::Reply(rpl) => println!("{}",rpl),
                    IoReply::Stum => (), 
                }
                Ok(CommandStatus::Done)
           }
       })
       .build().context("Failed to create REPL")?;

    repl.run().context("Critical REPL error")?;

    Ok(())
}
