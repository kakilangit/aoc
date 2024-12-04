use aoc::aoc2024::day03;
use criterion::{criterion_group, criterion_main, Criterion};

fn part_one(c: &mut Criterion) {
    let computer = day03::Computer::new_from_data().unwrap();
    c.bench_function("do_multiplication", |b| {
        b.iter(|| computer.do_multiplication())
    });
}

fn part_two(c: &mut Criterion) {
    let computer = day03::Computer::new_from_data().unwrap();

    c.bench_function("do_conditional_multiplication", |b| {
        b.iter(|| computer.do_conditional_multiplication())
    });
}

criterion_group!(benches, part_one, part_two);
criterion_main!(benches);
