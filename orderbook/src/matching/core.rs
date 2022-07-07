use crate::data::{fill::Fill, orders::BookOrder, side::Side};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MatchInfo {
    pub remain: u64,
    pub pivot: Option<u64>,
}

pub fn gmatch<'a, I, L>(
    it: I,
    liml: L,
    id: u32,
    qty: u64,
    fills: &mut Vec<Fill>,
    limit_price: Option<u64>) -> MatchInfo
where L: Fn(u64, u64) -> bool,
    I: Iterator<Item=(&'a u64,&'a mut Vec<BookOrder>)>
{
    let mut remaining_qty = qty;
    let mut pivot: Option<u64> = None;
    //let mut update_bid_ask = false;
    for (ask_price, queue) in it {
        if queue.is_empty() {
            continue;
        }
        //if (update_bid_ask || min_ask.is_none()) && !queue.is_empty() {
        //    ob.min_ask = Some(*ask_price);
        //    update_bid_ask = false;
        //}
        if let Some(lp) = limit_price {
            if liml(lp, *ask_price) {
                break;
            }
        }
        if remaining_qty == 0 {
            break;
        }
        let filled_qty = process_queue(queue, remaining_qty, id, Side::Buy, fills);
        if queue.is_empty() {
            pivot = Some(*ask_price);
            //update_bid_ask = true;

        }
        remaining_qty -= filled_qty;
    }

    //self.update_min_ask();
    MatchInfo { remain: remaining_qty, pivot } 
}

pub fn process_queue(
    opposite_orders: &mut Vec<BookOrder>,
    remaining_qty: u64,
    id: u32,
    side: Side,
    fills: &mut Vec<Fill>,
) -> u64 {
    let mut qty_to_fill = remaining_qty;
    let mut filled_qty = 0;
    let mut filled_index = None;

    for (index, head_order) in opposite_orders.iter_mut().enumerate() {
        if qty_to_fill == 0 {
            break;
        }
        let available_qty = head_order.qty;
        if available_qty == 0 {
            filled_index = Some(index);
            continue;
        }
        let traded_quantity: u64;
        let filled;

        if qty_to_fill >= available_qty {
            traded_quantity = available_qty;
            qty_to_fill -= available_qty;
            filled_index = Some(index);
            filled = true;
        } else {
            traded_quantity = qty_to_fill;
            qty_to_fill = 0;
            filled = false;
        }
        head_order.qty -= traded_quantity;
        fills.push(Fill {
            taker: id.into(),
            other: head_order.id,
            qty: traded_quantity,
            price: 0,
            taker_side: side,
            total_fill: filled,

        });
        filled_qty += traded_quantity;
    }
    if let Some(index) = filled_index {
        opposite_orders.drain(0..index + 1);
    }

    filled_qty
}
