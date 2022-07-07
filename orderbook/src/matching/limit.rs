use crate::data::{fill::Fill, orders::BookOrder, orders::OrderType, side::Side};
use crate::orderbook as ob;
use crate::matching::core;


pub fn limit(ob: &mut ob::OrderBook, id: u32, side: Side, qty: u64, price: u64) -> ob::Trade 
{
    let mut fills: Vec<Fill> = Vec::new();
    let mut bookid: Option<u64> = None;

    match side {
        Side::Buy => {
            let mi = core::gmatch(ob.asks.iter_mut(), |lp, p| { lp < p }, id, qty, &mut fills, Some(price));
            if mi.remain > 0 {
                let queue_capacity = 128; //self.default_queue_capacity;
                ob.bids
                    .entry(price)
                    .or_insert_with(|| Vec::with_capacity(queue_capacity))
                    .push(BookOrder {
                        id: ob.orderid,
                        qty: mi.remain,
                        kind: OrderType::LimitOrder,
                    });
                bookid = Some(ob.orderid);
                ob.orderid += 1;
                ob.max_bid = match ob.max_bid {
                    None => Some(price),
                    Some(b) if price > b => Some(price),
                    _ => ob.max_bid,
                };
            }
            if let Some(_) = mi.pivot {
                ob.asks.retain(|&_k, v| { ! v.is_empty() });
            } 
            ob::Trade { id: bookid, fills, qty: qty - mi.remain, remain: mi.remain }
        },
        Side::Sell => {
            let mi = core::gmatch(ob.bids.iter_mut().rev(), |lp, p| { lp > p }, id, qty, &mut fills, Some(price));
            if mi.remain > 0 {
                let queue_capacity = 128; //ob.default_queue_capacity;
                ob.asks
                    .entry(price)
                    .or_insert_with(|| Vec::with_capacity(queue_capacity))
                    .push(BookOrder {
                        id: ob.orderid,
                        qty: mi.remain,
                        kind: OrderType::LimitOrder,
                    });
                bookid = Some(ob.orderid);
                ob.orderid += 1;
                ob.min_ask = match ob.min_ask {
                    None => Some(price),
                    Some(a) if price < a => Some(price),
                    _ => ob.min_ask,
                };
            }
            if let Some(_) = mi.pivot {
                ob.bids.retain(|&_k, v| { ! v.is_empty() });
            } 
            ob::Trade { id: bookid, fills, qty: qty - mi.remain, remain: mi.remain }
        }
    }
}
