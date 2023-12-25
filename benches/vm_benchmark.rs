use criterion::{black_box, criterion_group, criterion_main, Criterion};

use advent_of_code_2019::vm::VM;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 9 quine", |b| {
        b.iter(|| {
            let input = vec![
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
            ];
            let mut vm = VM::new(black_box(input));
            vm.run();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
