use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_hpc_exercise::{
    cache_associativity::{sum_256_step, sum_257_step},
    reand_vec,
};

fn sum_step_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum_step");
    let mut vec = reand_vec(100000);
    group.bench_function("256", |b| b.iter(|| sum_256_step(black_box(&mut vec))));
    group.bench_function("257", |b| b.iter(|| sum_257_step(black_box(&mut vec))));
}

criterion_group!(benches, sum_step_benchmark);
criterion_main!(benches);
