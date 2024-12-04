use aoc::aoc2024::day02;
use criterion::{criterion_group, criterion_main, Criterion};

fn part_one(c: &mut Criterion) {
    let report = day02::Report::new_from_data().unwrap();
    c.bench_function("sum_of_safe_reports", |b| {
        b.iter(|| report.sum_of_safe_reports())
    });
}

fn part_two(c: &mut Criterion) {
    let report = day02::Report::new_from_data().unwrap();
    c.bench_function("sum_of_dampened_reports", |b| {
        b.iter(|| report.sum_of_dampened_reports())
    });
}

criterion_group!(benches, part_one, part_two);
criterion_main!(benches);
