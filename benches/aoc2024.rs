use aoc::aoc2024::day01;
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

criterion_group!(benches, bench_day_one);
criterion_main!(benches);
