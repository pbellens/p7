use crate::data::side::Side;
use crate::orderbook as ob;
use crate::matching::core;


pub fn market(ob: &mut ob::OrderBook, id: u32, side: Side, qty: u64) -> ob::Trade {
    let mut fills = Vec::new();
    match side {
        Side::Buy => {
            let mi = core::gmatch(ob.asks.iter_mut(), |lp, p| { lp < p }, id, qty, &mut fills, None);
            if let Some(_) = mi.pivot {
                ob.asks.retain(|&_k, v| { ! v.is_empty() });
            } 
            ob::Trade { fills, qty: qty - mi.remain }
        },
        Side::Sell => {
            let mi = core::gmatch(ob.bids.iter_mut().rev(), |lp, p| { lp > p }, id, qty, &mut fills, None);
            if let Some(_) = mi.pivot {
                ob.bids.retain(|&_k, v| { ! v.is_empty() });
            } 
            ob::Trade { fills, qty: qty - mi.remain }
        }
    }
}

