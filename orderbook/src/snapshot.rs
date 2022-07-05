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

impl std::fmt::Display for Snapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut d = "".to_owned();
        for p in self.asks.iter().zip_longest(self.bids.iter().rev()) {
            d.push_str(&match p {
                EitherOrBoth::Both(a, b) => format!("ask: {:>8} #{:<8} bid: {:>8} #{:<8}\n", a.price, a.qty, b.price, b.qty),
                EitherOrBoth::Left(a) => format!("ask: {:>8} #{:<8} bid: {:>8} #{:<8}\n", a.price, a.qty, '-', 0),
                EitherOrBoth::Right(b) => format!("ask: {:>8} #{:<8} bid: {:>8} #{:<8}\n", '-', 0, b.price, b.qty),
            });
        }
        if d.is_empty() {
            write!(f, "ask: {:>8} #{:<8} bid: {:>8} #{:<8}", '-', 0, '-', 0)
        } else {
            d.pop();
            write!(f, "{}", d)
        }
    }
}
