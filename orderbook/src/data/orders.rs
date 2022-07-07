use serde::{Serialize, Deserialize};
use rand::distributions::{Distribution, Standard, Uniform};
use super::side::Side;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum OrderType {
    LimitOrder,
    MarketOrder,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Order {
    pub qty: u64,
    pub price: u64,
    pub prod: u32,
    pub side: Side,
    pub kind: OrderType,
}

#[derive(Debug, Clone, Copy)]
pub struct BookOrder {
    pub id: u64,
    pub qty: u64,
    pub kind: OrderType,
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.kind {
            OrderType::LimitOrder => write!(f,  "limit : {} {:2}: {:4}$ #{:<8}", self.side, self.prod, self.price, self.qty),
            OrderType::MarketOrder => write!(f, "market: {} {:2}:       #{:<8}", self.side, self.prod, self.qty)
        }
    }
}


pub struct FlipIterator {
    ordertype: OrderType,
    side: Side,
}

impl FlipIterator {
    pub fn new(ordertype: OrderType) -> Self {
        FlipIterator {
            ordertype,
            side: Side::Sell,
        }
    }
}

impl Iterator for FlipIterator {
    type Item = Order;

    fn next(&mut self) -> Option<Self::Item> {
        let order = Order {
            qty: 100,
            price: 10,
            prod: 2,
            side: self.side,
            kind: self.ordertype
        };
        self.side = !self.side; 
        Some(order)
    }
}

pub struct RandomIterator {
    ordertype: OrderType,
    rng: rand::rngs::ThreadRng
}

impl RandomIterator {
    pub fn new(ordertype: OrderType) -> Self {
        RandomIterator {
            ordertype,
            rng: rand::thread_rng()
        }
    }
}

impl Distribution<Side> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Side {
        match rng.gen_range(0..=1) { 
            0 => Side::Buy,
            1 => Side::Sell,
            _ => Side::Sell
        }
    }
}

impl Iterator for RandomIterator {
    type Item = Order;

    fn next(&mut self) -> Option<Self::Item> {
        let price = Uniform::from(10..20);
        let order = Order {
            qty: 100,
            price: price.sample(&mut self.rng),
            prod: 2,
            side: rand::random(),
            kind: self.ordertype
        };
        Some(order)
    }
}
