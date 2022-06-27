use criterion::*;
use ::orderbook::data::orders;
use ::orderbook::orderbook;

pub fn one_to_one(c: &mut Criterion) {
    let mut group = c.benchmark_group("lean throughput");
    group.throughput(Throughput::Elements(1000000));
    group.bench_function(
        "lean", 
        |b| b.iter(|| {
            let mut ob = orderbook::OrderBook::new();
            orders::FlipIterator::new(orders::OrderType::LimitOrder)
                .take(1000000)
                .for_each(
                    |o| {
                        ob.execute(o);
                    })
        }));
    group.finish();
}

pub fn chaotic(c: &mut Criterion) {
    let mut group = c.benchmark_group("lean throughput");
    group.throughput(Throughput::Elements(1000000));
    group.bench_function(
        "lean", 
        |b| b.iter(|| {
            let mut ob = orderbook::OrderBook::new();

            orders::RandomIterator::new(orders::OrderType::LimitOrder)
                .take(1000000)
                .for_each(
                    |o| {
                        ob.execute(o);
                    })
        }));
    group.finish();
}

criterion_group!(benches, one_to_one, chaotic);
criterion_main!(benches);
