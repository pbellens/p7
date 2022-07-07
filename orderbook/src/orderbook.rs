use crate::matching::market;
use crate::matching::limit;
use crate::{data::fill, data::orders, data::trade, snapshot};
use std::collections::BTreeMap;
use std::fmt::Display;
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct OrderBook {
    pub last_trade: Option<trade::Trade>,
    pub min_ask: Option<u64>,
    pub max_bid: Option<u64>,
    pub orderid: u64,
    pub asks: BTreeMap<u64, Vec<orders::BookOrder>>,
    pub bids: BTreeMap<u64, Vec<orders::BookOrder>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
    pub id: Option<u64>,
    pub fills: Vec<fill::Fill>,
    pub qty: u64,
    pub remain: u64
}

impl Trade {
    pub fn new() -> Self {
        Trade { id: None, fills: vec![], qty: 0, remain: 0 }
    }
}

impl Display for Trade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = "".to_owned();
        if let Some(id) = self.id {
            d.push_str(&format!("\u{1F4D2} order ID: {} #{}\n", id, self.remain));
        } else {
            d.push_str(&format!("\u{1F340} qty: {}\n", self.qty));
        }
        d.pop();
        write!(f, "{}", d)
    }
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            last_trade: None,
            min_ask: None,
            max_bid: None,
            orderid: 0,
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
        }
    }

    pub fn execute(&mut self, order: orders::Order) -> Trade {
        match order {
            orders::Order {
                prod: p,
                qty: q,
                price: _,
                side: s,
                kind: orders::OrderType::MarketOrder,
            } => market::market(self, p, s, q),
            orders::Order {
                prod: p,
                qty: q,
                price: pr,
                side: s,
                kind: orders::OrderType::LimitOrder,
            } => limit::limit(self, p, s, q, pr),
        }
    }
}

impl std::fmt::Display for OrderBook {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = snapshot::Snapshot::new(self, 4);
        std::fmt::Display::fmt(&s, f)
    }
}
