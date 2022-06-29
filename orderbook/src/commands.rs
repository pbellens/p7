use serde::{Serialize, Deserialize};
use crate::data::orders;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Cmd {
    Order(orders::Order),
    Snapshot(usize)
}

