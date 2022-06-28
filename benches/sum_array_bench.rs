use criterion::{black_box, criterion_group, BenchmarkId, Criterion};
use rust_hpc_exercise::sum_array::{rand_arry, sum_array_with_bit_operator, sum_array_with_branch};
use criterion::criterion_main;

fn sum_array_with_branch_benchmark(c: &mut Criterion) {
    let unsort_array = rand_arry();
    let mut sort_array: [i32; 1000] = rand_arry();
    sort_array.sort();
    let threshold_array: [i32; 9] = [100, 200, 300, 400, 500, 600, 700, 800, 900];
    let mut group = c.benchmark_group("sum_array_with_branch");
    for threshold in threshold_array {
        group.bench_with_input(
            BenchmarkId::new("unsort_branch", threshold),
            &threshold,
            |b, threshold| b.iter(|| sum_array_with_branch(*threshold, black_box(unsort_array))),
        );
        group.bench_with_input(
            BenchmarkId::new("sort_branch", threshold),
            &threshold,
            |b, threshold| b.iter(|| sum_array_with_branch(*threshold, black_box(sort_array))),
        );
        group.bench_with_input(
            BenchmarkId::new("unsort_bit_operator", threshold),
            &threshold,
            |b, threshold| {
                b.iter(|| sum_array_with_bit_operator(*threshold, black_box(unsort_array)));
            },
        );
    }
}

criterion_group!(benches, sum_array_with_branch_benchmark);
criterion_main!(benches);