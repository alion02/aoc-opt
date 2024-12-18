#![feature(portable_simd)]

use std::{fs::read_to_string, hint::black_box};

use aocopt::day17::{part1, part2};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn day17(c: &mut Criterion) {
    let s = read_to_string("./inputs/17.txt").unwrap();
    let s = s.as_str();

    c.bench_function("day17 part1", |b| b.iter(|| part1(black_box(s))));
    c.bench_function("day17 part2", |b| b.iter(|| part2(black_box(s))));

    assert_eq!(part1(s).to_string(), read_to_string("./outputs/17p1.txt").unwrap());
    assert_eq!(part2(s).to_string(), read_to_string("./outputs/17p2.txt").unwrap());
}

criterion_group!(benches, day17);
criterion_main!(benches);
