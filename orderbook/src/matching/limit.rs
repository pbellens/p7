use crate::data::{fill::Fill, orders::Order, orders::OrderType, side::Side};
use crate::orderbook as ob;
use crate::matching::core;


pub fn limit(ob: &mut ob::OrderBook, id: u32, side: Side, qty: u64, price: u64) -> ob::Trade 
{
    let remaining_qty;
    let mut fills: Vec<Fill> = Vec::new();

    match side {
        Side::Buy => {
            remaining_qty = core::gmatch(ob.asks.iter_mut(), |lp, p| { lp < p }, id, qty, &mut fills, Some(price));
            if remaining_qty > 0 {
                let queue_capacity = 128; //self.default_queue_capacity;
                ob.bids
                    .entry(price)
                    .or_insert_with(|| Vec::with_capacity(queue_capacity))
                    .push(Order {
                        qty: remaining_qty,
                        price,
                        prod: 0,
                        side: Side::Buy,
                        kind: OrderType::LimitOrder,
                    });
                ob.max_bid = match ob.max_bid {
                    None => Some(price),
                    Some(b) if price > b => Some(price),
                    _ => ob.max_bid,
                };
            }
        }
        Side::Sell => {
            remaining_qty = core::gmatch(ob.bids.iter_mut(), |lp, p| { lp > p }, id, qty, &mut fills, Some(price));
            if remaining_qty > 0 {
                let queue_capacity = 128; //ob.default_queue_capacity;
                ob.asks
                    .entry(price)
                    .or_insert_with(|| Vec::with_capacity(queue_capacity))
                    .push(Order {
                        qty: remaining_qty,
                        price,
                        prod: 0,
                        side: Side::Sell,
                        kind: OrderType::LimitOrder,
                    });
                ob.min_ask = match ob.min_ask {
                    None => Some(price),
                    Some(a) if price < a => Some(price),
                    _ => ob.min_ask,
                };
            }
        }
    }

    ob::Trade { fills: fills, qty: qty - remaining_qty }
}
