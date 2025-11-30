use std::time::{SystemTime, UNIX_EPOCH};

use clob::{
    order::{Order, Side},
    order_book::OrderBook,
};
use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
    let mut order_book = OrderBook::new();
    let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

    c.bench_function("Limit order", |b| {
        b.iter(|| {
            order_book.submit_limit_order(Order {
                id: 1,
                user_id: 1,
                side: Side::Bid,
                price: 100,
                quantity: 10,
                timestamp,
            })
        })
    });

}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);