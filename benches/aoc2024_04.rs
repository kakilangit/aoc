use aoc::aoc2024::day04;
use criterion::{criterion_group, criterion_main, Criterion};

fn part_one(c: &mut Criterion) {
    let mut monitor = day04::ElfMonitor::new_from_data().unwrap();
    monitor.calculate_vectors();

    c.bench_function("count_xmas", |b| b.iter(|| monitor.count_xmas()));
}

criterion_group!(benches, part_one);
criterion_main!(benches);
