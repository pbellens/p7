use crate::data::{fill::Fill, orders::Order, orders::OrderType, side::Side};
use crate::orderbook as ob;
use crate::matching::core;


pub fn market(ob: &mut ob::OrderBook, id: u32, side: Side, qty: u64) -> ob::Trade {
    let remaining_qty;
    let mut fills = Vec::new();

    match side {
        Side::Buy => {
            remaining_qty = core::gmatch(ob.asks.iter_mut(), |lp, p| { lp < p }, id, qty, &mut fills, None);
        }
        Side::Sell => {
            remaining_qty = core::gmatch(ob.bids.iter_mut().rev(), |lp, p| { lp > p }, id, qty, &mut fills, None);
        }
    }

    ob::Trade { fills: fills, qty: qty - remaining_qty }
}

