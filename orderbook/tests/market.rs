use orderbook::orderbook as ob;
use orderbook::data::{orders, side, };

#[test]
fn single_buy_does_not_match() {
    let mut ob = ob::OrderBook::new();
    let order = orders::Order {
        qty: 10,
        price: 100,
        prod: 2,
        side: side::Side::Buy,
        kind: orders::OrderType::LimitOrder};
    let trade = ob.execute(order);
    assert!(trade.fills.is_empty());
    assert_eq!(0, trade.qty);
}

#[test]
fn single_sell_does_not_match() {
    let mut ob = ob::OrderBook::new();
    let order = orders::Order {
        qty: 10,
        price: 100,
        prod: 2,
        side: side::Side::Sell,
        kind: orders::OrderType::LimitOrder};
    let trade = ob.execute(order);
    assert!(trade.fills.is_empty());
    assert_eq!(0, trade.qty);
}
