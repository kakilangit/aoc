use aoc::aoc2024::{day01, day02};
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_day_one(c: &mut Criterion) {
    let location = day01::Location::new_from_data().unwrap();
    c.bench_function("sum_of_difference", |b| {
        b.iter(|| location.sum_of_difference().unwrap())
    });

    c.bench_function("sum_of_similarities", |b| {
        b.iter(|| location.sum_of_similarities().unwrap())
    });
}

fn bench_day_two(c: &mut Criterion) {
    let report = day02::Report::new_from_data().unwrap();
    c.bench_function("sum_of_safe_reports", |b| {
        b.iter(|| report.sum_of_safe_reports())
    });

    c.bench_function("sum_of_dampened_reports", |b| {
        b.iter(|| report.sum_of_dampened_reports())
    });
}

criterion_group!(benches, bench_day_one, bench_day_two);
criterion_main!(benches);
