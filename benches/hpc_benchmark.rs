use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rust_hpc_exercise::{rand_arry, sum_array_with_branch};

fn sum_array_with_branch_benchmark(c: &mut Criterion) {
    let array = rand_arry();
    let threshold_array: [u32; 9] = [10, 20, 30, 40, 50, 60, 70, 80, 90];
    let mut group = c.benchmark_group("sum_array_with_branch");
    for threshold in threshold_array {
        group.bench_function(BenchmarkId::from_parameter(threshold), |b| {
            b.iter(|| sum_array_with_branch(black_box(threshold), black_box(array)))
        });
    }
}

criterion_group!(benches, sum_array_with_branch_benchmark);
criterion_main!(benches);
