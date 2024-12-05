use aoc::aoc2024::day04;
use criterion::{criterion_group, criterion_main, Criterion};

fn part_one(c: &mut Criterion) {
    let monitor = day04::ElfMonitor::new_from_data().unwrap();

    c.bench_function("count_xmas_improved", |b| b.iter(|| monitor.count_xmas()));
}

fn part_two(c: &mut Criterion) {
    let monitor = day04::ElfMonitor::new_from_data().unwrap();

    c.bench_function("count_crossmas", |b| b.iter(|| monitor.count_crossmas()));
}
criterion_group!(benches, part_one, part_two);
criterion_main!(benches);
