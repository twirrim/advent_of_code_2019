use log::{debug, info};
use simple_logger::SimpleLogger;

use advent_of_code_2019::read_file;
use advent_of_code_2019::vm::VM;

fn part_one(input: &Vec<usize>) {
    /*
    Once you have a working computer, the first step is to
    restore the gravity assist program (your puzzle input)
    to the "1202 program alarm" state it had just before
    the last computer caught fire. To do this, before running
    the program, replace position 1 with the value 12 and
    replace position 2 with the value 2. What value is left
    at position 0 after the program halts?
     */
    let mut vm = VM::new(input.clone());
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
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    info!("Reading and parsing input took: {:?}", start.elapsed());

    let part_one_start = std::time::Instant::now();
    part_one(&input);
    info!("Part one took: {:?}", part_one_start.elapsed());
    /*
    let part_two_start = std::time::Instant::now();
    part_two(&input);
    info!("Part two took: {:?}", part_two_start.elapsed());
    */
    info!("Overall time take: {:?}", start.elapsed());
}
