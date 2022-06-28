use crate::orderbook;
use itertools::{EitherOrBoth, Itertools};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct BookLevel {
    pub price: u64,
    pub qty: u64
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Snapshot {
    pub bids: Vec<BookLevel>,
    pub asks: Vec<BookLevel>
}

impl Snapshot {
    pub fn new(ob: &orderbook::OrderBook, depth: usize) -> Self {
        let mut bids: Vec<BookLevel> = Vec::with_capacity(depth);
        let mut asks: Vec<BookLevel> = Vec::with_capacity(depth);

        for p in ob.asks.iter().zip_longest(ob.bids.iter().rev()).take(depth) 
        {
            match p {
                EitherOrBoth::Both(a, b) => {
                    asks.push(
                        BookLevel{ 
                            price: *a.0, 
                            qty: a.1.iter().fold(0, |acc, &o| acc + o.qty),
                        });
                    bids.push(
                        BookLevel{ 
                            price: *b.0, 
                            qty: b.1.iter().fold(0, |acc, &o| acc + o.qty)
                        });
                },
                EitherOrBoth::Left(a) => {
                    asks.push(
                        BookLevel{ 
                            price: *a.0, 
                            qty: a.1.iter().fold(0, |acc, &o| acc + o.qty),
                    });
                },
                EitherOrBoth::Right(b) => {
                    bids.push(
                        BookLevel{ 
                            price: *b.0, 
                            qty: b.1.iter().fold(0, |acc, &o| acc + o.qty)
                        });
                }
            };
        }

        Snapshot { bids, asks }
    }
}
