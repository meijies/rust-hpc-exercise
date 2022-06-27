use criterion::criterion_main;
mod benchmarks;
criterion_main!(benchmarks::sum_array_bench::benches);
