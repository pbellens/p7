use super::side::Side;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct Fill {
    /// The ID of the order that triggered the fill.
    pub taker: u128,
    /// The ID of the matching order.
    pub other: u128,
    /// The quantity that was traded.
    pub qty: u64,
    /// The price at which the trade happened.
    pub price: u64,
    /// The side of the taker order (order 1)
    pub taker_side: Side,
    /// Whether this order was a total (true) or partial (false) fill of the
    /// maker order.
    pub total_fill: bool,
}
