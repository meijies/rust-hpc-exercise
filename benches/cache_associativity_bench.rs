use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rust_hpc_exercise::{
    cache_associativity::{
        init_array_1, init_array_1_prefetch, init_array_2, init_array_2_prefetch, walk_step,
        walk_step_prefetch,
    },
    reand_vec,
};

fn sum_step_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum_step");
    let mut vec = reand_vec(100000);
    let step_array: [usize; 10] = [2, 4, 5, 8, 16, 32, 64, 255, 256, 257];
    for step in step_array {
        group.bench_with_input(BenchmarkId::new("walk_step", step), &step, |b, step| {
            b.iter(|| walk_step(*step, black_box(&mut vec)));
        });
        group.bench_with_input(
            BenchmarkId::new("walk_step_prefech", step),
            &step,
            |b, step| {
                b.iter(|| walk_step_prefetch(*step, black_box(&mut vec)));
            },
        );
    }
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
