use crate::data::side;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Fill {
    /// The ID of the order that triggered the fill (taker).
    pub order_1: u64,
    /// The ID of the matching order.
    pub order_2: u64,
    /// The quantity that was traded.
    pub qty: u64,
    /// The price at which the trade happened.
    pub price: u64,
    /// The side of the taker order (order 1)
    pub taker_side: side::Side,
    /// Whether this order was a total (true) or partial (false) fill of the
    /// maker order.
    pub total_fill: bool,
}
