use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use subset_sum::{get_subset_sum, SubsetSumResult};

fn benchmark_subset_sum(c: &mut Criterion) {
    // Define a sample list and sum for benchmarking
    let list = vec![3, 34, 4, 12, 5, 2];
    let sum = 9;
    let timeout_in_ms = None;

    // Benchmark the get_subset_sum function
    c.bench_function("get_subset_sum", |b| {
        b.iter(|| {
            let result: SubsetSumResult<i32> = get_subset_sum(list.clone(), sum, timeout_in_ms);
            criterion::black_box(result)
        });
    });
}

criterion_group!(benches, benchmark_subset_sum);
criterion_main!(benches);
