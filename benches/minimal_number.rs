use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;

use coins::minimal_number;
use coins::inputs;

fn benchmark_50_100_2_7_100(c: &mut Criterion) {
    let (label, inputs) = inputs::generate(10000, 50, 100, 2, 7, 100);
    let inputs = &inputs;

    c.bench_function(&label, move |b|
        b.iter(|| inputs.into_iter().for_each(|input| {
            minimal_number::calculate(black_box(input.amount), black_box(&input.coins));
        })));
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(10)
        .measurement_time(Duration::from_secs(5))
        .warm_up_time(Duration::from_secs(2));
    targets = benchmark_50_100_2_7_100
}
criterion_main!(benches);