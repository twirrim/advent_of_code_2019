use log::{debug, info};
use simple_logger::SimpleLogger;

use advent_of_code_2019::read_file;

#[inline(always)]
fn calculate_fuel_needed(input: isize) -> isize {
    input / 3 - 2
}

fn part_one(input: &Vec<isize>) {
    let answer = input
        .iter()
        .map(|x| calculate_fuel_needed(*x))
        .collect::<Vec<isize>>();
    debug!("{:?}", answer);
    info!("Part one: {}", answer.iter().sum::<isize>());
}

fn part_two(input: &Vec<isize>) {
    let mut total_to_add: isize = 0;
    for module in input {
        let mut fuel_to_add = 0;
        let mut to_add = *module;
        loop {
            to_add = calculate_fuel_needed(to_add);
            if to_add <= 0 {
                break;
            }
            fuel_to_add += to_add;
        }
        total_to_add += fuel_to_add;
    }
    info!("Part two: {}", total_to_add);
}

fn main() {
    let start = std::time::Instant::now();
    SimpleLogger::new().env().init().unwrap();
    info!("Reading input");
    let input = read_file("./input/day1")
        .iter()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_fuel_needed() {
        let test_cases: Vec<(isize, isize)> = vec![(12, 2), (14, 2), (1969, 654), (100756, 33583)];

        for test_case in test_cases {
            assert_eq!(calculate_fuel_needed(test_case.0), test_case.1);
        }
    }
}
