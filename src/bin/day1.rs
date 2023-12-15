use std::f32::consts::PI;

use log::{debug, info};
use simple_logger::SimpleLogger;

use advent_of_code_2019::read_file;

fn calculate_fuel_needed(input: &usize) -> usize {
    input / 3 - 2
}

fn part_one(input: &Vec<usize>) {
    let answer = input
        .iter()
        .map(calculate_fuel_needed)
        .collect::<Vec<usize>>();
    debug!("{:?}", answer);
    info!("Part one: {}", answer.iter().sum::<usize>());
}

fn main() {
    let start = std::time::Instant::now();
    SimpleLogger::new().env().init().unwrap();
    info!("Reading input");
    let input = read_file("./input/day1")
        .iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    info!("Reading and parsing input took: {:?}", start.elapsed());

    let part_one_start = std::time::Instant::now();
    part_one(&input);
    info!("Part one took: {:?}", part_one_start.elapsed());

    info!("Overall time take: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_fuel_needed() {
        let test_cases: Vec<(usize, usize)> = vec![(12, 2), (14, 2), (1969, 654), (100756, 33583)];

        for test_case in test_cases {
            assert_eq!(calculate_fuel_needed(&test_case.0), test_case.1);
        }
    }
}
