use log::info;
use simple_logger::SimpleLogger;

use advent_of_code_2019::vm::VM;
use advent_of_code_2019::{debug_println, read_file};

fn part_two(input: &[isize]) {}

fn part_one(input: &[isize]) {
    let mut vm = VM::new(input.to_owned());
    debug_println!("{:?}", vm);
    vm.run();
}

fn main() {
    let start = std::time::Instant::now();
    SimpleLogger::new().env().init().unwrap();
    info!("Reading input");
    // Only a single line in the input
    let input = read_file("./input/day5")[0]
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
