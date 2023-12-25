use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lazy_static::lazy_static;

use advent_of_code_2019::vm::VM;

lazy_static! {
    static ref QUINE: Vec<isize> =
        vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99];
    static ref DAY_9: (Vec<isize>, isize) = {
        let input = include_str!("../input/day9")
            .split(",")
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();

        (input, 2)
    };
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 9 quine", |b| {
        b.iter(|| {
            let mut vm = VM::new(QUINE.clone());
            vm.run();
        })
    });
    c.bench_function("day 9 full", |b| {
        b.iter(|| {
            let mut vm = VM::new(DAY_9.0.clone());
            vm.push_input(DAY_9.1);
            vm.run();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
