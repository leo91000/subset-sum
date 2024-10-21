use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use subset_sum::{get_all_subset_sums, SubsetSumAllResult};

fn benchmark_all_subset_sums(c: &mut Criterion) {
    // Define a sample list and sum for benchmarking
    let list = vec![3, 34, 4, 12, 5, 2];
    let sum = 9;
    let timeout_in_ms = None;

    // Benchmark the get_all_subset_sums function
    c.bench_function("get_all_subset_sums", |b| {
        b.iter(|| {
            let result: SubsetSumAllResult<i32> =
                get_all_subset_sums(list.clone(), sum, timeout_in_ms);
            criterion::black_box(result)
        });
    });
}

criterion_group!(benches, benchmark_all_subset_sums);
criterion_main!(benches);
