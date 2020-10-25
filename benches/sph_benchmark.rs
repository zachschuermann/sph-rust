use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sph;

fn sim(n: u32, i: u32) {
    let state = &mut sph::State::new(n).unwrap();
    for _ in 0..i {
        state.update();
    }
}

fn sph_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample-size-10");
    group.sample_size(10);
    group.bench_function("simulation: n=1200, i=1000", |b| b.iter(|| sim(black_box(1200), black_box(1000))));
    group.finish();
}

criterion_group!(benches, sph_benchmark);
criterion_main!(benches);
