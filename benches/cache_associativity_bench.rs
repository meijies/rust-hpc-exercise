use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_hpc_exercise::{
    cache_associativity::{
        init_array_1, init_array_1_prefetch, init_array_2, init_array_2_prefetch, sum_256_step,
        sum_257_step,
    },
    reand_vec,
};

fn sum_step_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum_step");
    let mut vec = reand_vec(100000);
    group.bench_function("256", |b| b.iter(|| sum_256_step(black_box(&mut vec))));
    group.bench_function("257", |b| b.iter(|| sum_257_step(black_box(&mut vec))));
}

fn init_array_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("init_array");
    group.bench_function("1", |b| b.iter(|| init_array_1()));
    group.bench_function("2", |b| b.iter(|| init_array_2()));
    group.bench_function("1_prefetch", |b| b.iter(|| init_array_1_prefetch()));
    group.bench_function("2_prefetch", |b| b.iter(|| init_array_2_prefetch()));
}

criterion_group!(benches, sum_step_benchmark, init_array_benchmark);
criterion_main!(benches);
