use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use primes::miller_rabin::get_prime;

fn bench_primes(c: &mut Criterion) {
    c.bench_function("generate a prime", |b| {
        b.iter(|| {
            black_box(get_prime());
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(1_000);
    targets = bench_primes
}

criterion_main!(benches);
