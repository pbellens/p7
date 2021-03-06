extern crate orderbook;
use orderbook::data::orders;
use orderbook::orderbook as ob;
use serde_json::Deserializer;
use std::fs::File;
use std::io::{BufReader, Error};

fn main() -> Result<(), Error> {
    let mut book = ob::OrderBook::new();

    //let buy = Order{ prod: 2, qty: 10, price: 2, side: side::Side::Buy, kind: OrderType::LimitOrder };
    //let sell = Order{ prod: 2, qty: 30, price: 4, side: side::Side::Sell, kind: OrderType::LimitOrder };

    let f = File::open("/home/pbellens/git/p7/data/orders.json")?;
    let os = Deserializer::from_reader(BufReader::new(f));

    println!("{}", book);
    let _trades: Vec<ob::Trade> = os.into_iter::<orders::Order>()
        .into_iter()
        .fold(
            vec![],
            |mut es, order| {
                let o = order.unwrap();
                println!("\t\t{}", o);
                let trade = book.execute(o);
                es.push(trade);
                println!("{}", book);
                es
            }
        );

    Ok(())
}
