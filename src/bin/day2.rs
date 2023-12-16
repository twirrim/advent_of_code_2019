use log::{debug, info};
use simple_logger::SimpleLogger;

use advent_of_code_2019::read_file;
use advent_of_code_2019::vm::VM;

fn part_two(input: &[isize]) {
    /*
    To complete the gravity assist, you need to determine what
    pair of inputs produces the output 19690720.

    The inputs should still be provided to the program by
    replacing the values at addresses 1 and 2, just like before.

    In this program, the value placed in address 1 is called the
    noun, and the value placed in address 2 is called the verb.
    Each of the two input values will be between 0 and 99, inclusive.

    Find the input noun and verb that cause the program to produce
    the output 19690720. What is 100 * noun + verb?
     */
    let vm = VM::new(input.to_owned(), None);
    let mut finished = false;
    for noun in 0..=99 {
        if !finished {
            for verb in 0..=99 {
                let mut test_vm = vm.clone();
                test_vm.set_memory(1, noun);
                test_vm.set_memory(2, verb);
                test_vm.run();
                if test_vm.get_memory(0) == 19_690_720 {
                    finished = true;
                    let answer = 100 * noun + verb;
                    info!("Part two: {answer}");
                    break;
                }
            }
        }
    }
}

fn part_one(input: &[isize]) {
    /*
    Once you have a working computer, the first step is to
    restore the gravity assist program (your puzzle input)
    to the "1202 program alarm" state it had just before
    the last computer caught fire. To do this, before running
    the program, replace position 1 with the value 12 and
    replace position 2 with the value 2. What value is left
    at position 0 after the program halts?
     */
    let mut vm = VM::new(input.to_owned(), None);
    debug! {"{:?}", vm};
    // replace position 1 with the value 12
    vm.set_memory(1, 12);
    debug! {"{:?}", vm};
    // replace position 2 with the value 2
    vm.set_memory(2, 2);
    debug! {"{:?}", vm};
    vm.run();
    info!("Part 1 answer: {}", vm.get_memory(0));
}

fn main() {
    let start = std::time::Instant::now();
    SimpleLogger::new().env().init().unwrap();
    info!("Reading input");
    // Only a single line in the input
    let input = read_file("./input/day2")[0]
        .split(',')
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    info!("Reading and parsing input took: {:?}", start.elapsed());

    let part_one_start = std::time::Instant::now();
    part_one(&input);
    info!("Part one took: {:?}", part_one_start.elapsed());

    let part_two_start = std::time::Instant::now();
    part_two(&input);
    info!("Part two took: {:?}", part_two_start.elapsed());

    info!("Overall time take: {:?}", start.elapsed());
}
