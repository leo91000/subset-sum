use codspeed_criterion_compat::{black_box, criterion_group, criterion_main, Criterion};
use subset_sum::{get_all_subset_sums, SubsetSumAllResult};

fn benchmark_all_subset_sums(c: &mut Criterion) {
    let mut group = c.benchmark_group("all_subset_sums");

    group.bench_function("original case", |b| {
        b.iter(|| {
            let result: SubsetSumAllResult<i32> =
                get_all_subset_sums(vec![3, 34, 4, 12, 5, 2], 9, None);
            black_box(result)
        });
    });

    group.bench_function("small list, multiple solutions", |b| {
        b.iter(|| {
            let result: SubsetSumAllResult<i32> = get_all_subset_sums(vec![1, 2, 3, 4, 5], 5, None);
            black_box(result)
        });
    });

    group.bench_function("medium list, few solutions", |b| {
        b.iter(|| {
            let result: SubsetSumAllResult<i32> = get_all_subset_sums((1..20).collect(), 38, None);
            black_box(result)
        });
    });

    group.bench_function("list with duplicates", |b| {
        b.iter(|| {
            let result: SubsetSumAllResult<i32> =
                get_all_subset_sums(vec![1, 2, 2, 3, 3, 4, 5], 6, None);
            black_box(result)
        });
    });

    group.bench_function("no solution case", |b| {
        b.iter(|| {
            let result: SubsetSumAllResult<i32> =
                get_all_subset_sums(vec![10, 20, 30, 40], 5, None);
            black_box(result)
        });
    });

    group.bench_function("large list with timeout", |b| {
        b.iter(|| {
            let result: SubsetSumAllResult<i32> =
                get_all_subset_sums((1..30).collect(), 100, Some(100));
            black_box(result)
        });
    });

    group.finish();
}

criterion_group!(benches, benchmark_all_subset_sums);
criterion_main!(benches);
