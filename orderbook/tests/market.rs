use orderbook::orderbook as ob;
use orderbook::snapshot::Snapshot;
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

    let s = Snapshot::new(&ob, 1);
    assert_eq!(0, s.asks.len());
    assert_eq!(1, s.bids.len());
    assert_eq!(100, (&s.bids[0..=0])[0].price);
    assert_eq!(10, (&s.bids[0..=0])[0].qty);
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

    let s = Snapshot::new(&ob, 1);
    assert_eq!(1, s.asks.len());
    assert_eq!(0, s.bids.len());
    assert_eq!(100, (&s.asks[0..=0])[0].price);
    assert_eq!(10, (&s.asks[0..=0])[0].qty);
}

#[test]
fn single_buy_crossing_a_single_sell_matches() {
    let mut ob = ob::OrderBook::new();
    let sell = orders::Order {
        qty: 10,
        price: 100,
        prod: 2,
        side: side::Side::Sell,
        kind: orders::OrderType::LimitOrder};
    let buy = orders::Order {
        qty: 10,
        price: 100,
        prod: 2,
        side: side::Side::Buy,
        kind: orders::OrderType::LimitOrder};

    ob.execute(sell);
    let trade = ob.execute(buy);
    assert_eq!(1, trade.fills.len());
    assert_eq!(10, trade.qty);

    let s = Snapshot::new(&ob, 1);
    assert_eq!(0, s.bids.len());
    assert_eq!(0, s.asks.len());
}

#[test]
fn single_sell_crossing_a_single_buy() {
    let mut ob = ob::OrderBook::new();
    let sell = orders::Order {
        qty: 10,
        price: 100,
        prod: 2,
        side: side::Side::Sell,
        kind: orders::OrderType::LimitOrder};
    let buy = orders::Order {
        qty: 10,
        price: 100,
        prod: 2,
        side: side::Side::Buy,
        kind: orders::OrderType::LimitOrder};

    ob.execute(buy);
    let trade = ob.execute(sell);
    assert_eq!(1, trade.fills.len());
    assert_eq!(10, trade.qty);
    println!("orderbook is {:?}", ob);

    let s = Snapshot::new(&ob, 1);
    assert_eq!(0, s.bids.len());
    assert_eq!(0, s.asks.len());
}
