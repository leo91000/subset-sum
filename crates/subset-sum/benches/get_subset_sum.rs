use codspeed_criterion_compat::{black_box, criterion_group, criterion_main, Criterion};
use subset_sum::{get_subset_sum, SubsetSumResult};

fn benchmark_subset_sum(c: &mut Criterion) {
    let mut group = c.benchmark_group("subset_sum");

    group.bench_function("original case", |b| {
        b.iter(move || {
            let result: SubsetSumResult<i32> = get_subset_sum(vec![3, 34, 4, 12, 5, 2], 9, None);
            black_box(result)
        });
    });

    group.bench_function("medium list, small sum", |b| {
        b.iter(move || {
            let result: SubsetSumResult<i32> = get_subset_sum((1..100).collect(), 50, None);
            black_box(result)
        });
    });

    group.bench_function("small list, large sum (no solution)", |b| {
        b.iter(move || {
            let result: SubsetSumResult<i32> = get_subset_sum(vec![1, 2, 5, 10], 1000, None);
            black_box(result)
        });
    });

    group.bench_function("medium list, medium sum", |b| {
        b.iter(move || {
            let result: SubsetSumResult<i32> =
                get_subset_sum((1..101).map(|x| x * 2).collect(), 500, None);
            black_box(result)
        });
    });

    group.bench_function("list with negative numbers", |b| {
        b.iter(move || {
            let result: SubsetSumResult<i32> =
                get_subset_sum(vec![-10, -5, 0, 5, 10, 15, 20, 25], 15, None);
            black_box(result)
        });
    });

    group.bench_function("large list with timeout", |b| {
        b.iter(move || {
            let result: SubsetSumResult<i32> =
                get_subset_sum((1..10001).collect(), 50000, Some(100));
            black_box(result)
        });
    });

    group.finish();
}

criterion_group!(benches, benchmark_subset_sum);
criterion_main!(benches);
