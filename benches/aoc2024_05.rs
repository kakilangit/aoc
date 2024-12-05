use aoc::aoc2024::day05;
use criterion::{criterion_group, criterion_main, Criterion};

fn part_one(c: &mut Criterion) {
    let printer = day05::ElfPrinter::new_from_data().unwrap();

    c.bench_function("sum_of_correct_ordered", |b| {
        b.iter(|| printer.sum_of_correct_ordered())
    });
}

fn part_two(c: &mut Criterion) {
    let printer = day05::ElfPrinter::new_from_data().unwrap();

    c.bench_function("sum_of_after_topological_sort", |b| {
        b.iter(|| printer.sum_of_after_topological_sort())
    });
}
criterion_group!(benches, part_one, part_two);
criterion_main!(benches);
