use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rust_hpc_exercise::{rand_arry, sum_array_with_branch};

fn sum_array_with_branch_benchmark(c: &mut Criterion) {
    let unsort_array = rand_arry();
    let mut sort_array: [u32; 1000] = rand_arry();
    sort_array.sort();
    let threshold_array: [u32; 9] = [100, 200, 300, 400, 500, 600, 700, 800, 900];
    let mut group = c.benchmark_group("sum_array_with_branch");
    for threshold in threshold_array {
        group.bench_with_input(
            BenchmarkId::new("unsort", threshold),
            &threshold,
            |b, threshold| b.iter(|| sum_array_with_branch(*threshold, black_box(unsort_array))),
        );
        group.bench_with_input(
            BenchmarkId::new("sort", threshold),
            &threshold,
            |b, threshold| b.iter(|| sum_array_with_branch(*threshold, black_box(sort_array))),
        );
    }
}

criterion_group!(benches, sum_array_with_branch_benchmark);
criterion_main!(benches);
