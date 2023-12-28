use std::cmp::{max, min};
use std::collections::HashMap;

use image::{imageops, ImageBuffer, RgbImage};
use log::info;
use simple_logger::SimpleLogger;

use advent_of_code_2019::vm::VM;
use advent_of_code_2019::{debug_println, read_file, Direction, Point};

/*
 For this puzzle, need to "paint" an ID on the hull.  The hull is a 2D grid.
 Panels start black.  The VM program input will tell us what to paint, and how to turn (90 degrees either left or right)
 On each round the robot will:
 * Tell the VM what colour the current panel is (0 black, 1 white)
 * Read two output values.
 ** First is what colour to paint the panel (0 black, 1 white)
 ** Second is what way to turn 90 degrees (0 left, 1 right)
 * Move forwards one cell
*/

// Paint Robot?  Paint Robot!
struct PaintRobot {
    location: Point<isize>,
    heading: Direction,
}

impl PaintRobot {
    // I should just have this as default, and make new take a starting Point
    fn new() -> Self {
        PaintRobot {
            location: Point { x: 0, y: 0 },
            heading: Direction::North,
        }
    }

    fn turn(&mut self, direction: isize) {
        match direction {
            0 => {
                debug_println!("Got {direction}. Turning Left");
                self.heading = self.heading.turn_left()
            }
            1 => {
                debug_println!("Got {direction}.  Turning Right");
                self.heading = self.heading.turn_right()
            }
            _ => panic!("{direction} not valid!"),
        }
    }

    fn move_robot(&mut self) {
        match self.heading {
            Direction::North => self.location += Point { x: 0, y: 1 },
            Direction::South => self.location += Point { x: 0, y: -1 },
            Direction::East => self.location += Point { x: 1, y: 0 },
            Direction::West => self.location += Point { x: -1, y: 0 },
        };
        debug_println!(
            "Location after moving {:?}: {:?}",
            self.heading,
            self.location
        );
    }
}

fn part_one(program: &[isize]) {
    /*
    Before you deploy the robot, you should probably have an estimate of the area it will cover:
    specifically, you need to know the number of panels it paints at least once, regardless of color.
    In the example above [on the puzzle page], the robot painted 6 panels at least once.
    (It painted its starting panel twice, but that panel is still only counted once;
    it also never painted the panel it ended on.)
    */

    // Using a HashSet, every time we paint, we'll add to the HashSet, as that'll dedupe.
    // Final answer will be HashSet length.
    let mut vm = VM::new(program.to_owned());
    let mut robot = PaintRobot::new();

    // Starting location is white
    let mut map: HashMap<Point<isize>, isize> = HashMap::from([(robot.location.clone(), 0)]);

    // Running the VM should see it end at a WaitingForInput state, which we can then build the loop around
    vm.run();
    while vm.needs_input() {
        let current_colour: isize = match map.get(&robot.location) {
            Some(colour) => *colour,
            None => {
                map.insert(robot.location.clone(), 0);
                0
            } // Default to black
        };

        debug_println!("Pushing to Input: {current_colour}");
        vm.push_input(current_colour);

        // Then run, and it should give me two outputs
        vm.run();

        // First we paint (for part 1, really doesn't matter what)
        if let Some(wanted_colour) = vm.pop_front_output() {
            *map.entry(robot.location.clone()).or_insert(wanted_colour) = wanted_colour;
        } else {
            panic!("Didn't get a paint output!")
        }

        // Then we turn
        if let Some(turn) = vm.pop_front_output() {
            robot.turn(turn);
        } else {
            panic!("Didn't get turn output!")
        }

        // Then we move
        robot.move_robot();

        // Then we run the robot, which should take us back to the start of the loop
        vm.run();
    }
    make_image_from_map(&map, "day_11_part_one.png");
    info!("{:?}", map.len());
}

fn part_two(program: &[isize]) {
    /*
    Based on the Space Law Space Brochure that the Space Police attached to one of your windows,
    a valid registration identifier is always eight capital letters. After starting the robot on
    a single white panel instead, what registration identifier does it paint on your hull?
    */
    let mut vm = VM::new(program.to_owned());
    let mut robot = PaintRobot::new();
    // Starting location is white
    let mut map: HashMap<Point<isize>, isize> = HashMap::from([(robot.location.clone(), 1)]);
    // Running the VM should see it end at a WaitingForInput state, which we can then build the loop around
    vm.run();
    while vm.needs_input() {
        let current_colour: isize = match map.get(&robot.location) {
            Some(colour) => *colour,
            None => {
                map.insert(robot.location.clone(), 0);
                0
            } // Default to black
        };

        debug_println!("Pushing to Input: {current_colour}");
        vm.push_input(current_colour);

        // Then run, and it should give me two outputs
        vm.run();

        // First we paint (for part 1, really doesn't matter what)
        if let Some(wanted_colour) = vm.pop_front_output() {
            *map.entry(robot.location.clone()).or_insert(wanted_colour) = wanted_colour;
        } else {
            panic!("Didn't get a paint output!")
        }

        // Then we turn
        if let Some(turn) = vm.pop_front_output() {
            robot.turn(turn);
        } else {
            panic!("Didn't get turn output!")
        }

        // Then we move
        robot.move_robot();

        // Then we run the robot, which should take us back to the start of the loop
        vm.run();
    }
    make_image_from_map(&map, "day_11_part_two.png");
}

fn make_image_from_map(map: &HashMap<Point<isize>, isize>, name: &str) {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for key in map.keys() {
        min_x = min(min_x, key.x);
        min_y = min(min_y, key.y);
        max_x = max(max_x, key.x);
        max_y = max(max_y, key.y);
    }
    let offset_x = 0 - min_x;
    let offset_y = 0 - min_y;

    // Make an RGB buffer with dimensions reflecting map
    let mut img: RgbImage =
        ImageBuffer::new((max_x + offset_x + 1) as u32, (max_y + offset_y + 1) as u32);

    // Make everything black
    for pixel in img.pixels_mut() {
        *pixel = image::Rgb([0, 0, 0]);
    }

    for (key, val) in map.iter() {
        let pixel = img.get_pixel_mut((key.x + offset_x) as u32, (key.y + offset_y) as u32);
        match val {
            0 => *pixel = image::Rgb([0, 0, 0]),
            1 => *pixel = image::Rgb([255, 255, 255]),
            _ => panic!("What? {val}"),
        }
    }

    let flipped_img = imageops::flip_vertical(&img);
    // Scale image
    let scaled_flipped_img = imageops::resize(
        &flipped_img,
        ((max_x + offset_x + 1) * 8) as u32,
        ((max_y + offset_y + 1) * 8) as u32,
        imageops::FilterType::Nearest,
    );
    info!("Saving image {name}");
    scaled_flipped_img.save(name).unwrap();
}

fn main() {
    let start = std::time::Instant::now();
    SimpleLogger::new().env().init().unwrap();
    info!("Reading input");
    // Only a single line in the input
    let input = read_file("./input/day11")[0]
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

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[test]
    fn test_facing_north() {
        let robot = PaintRobot::new();
        // New robots should be facing north
        assert_eq!(robot.heading, Direction::North);
    }

    #[rstest]
    #[case(Direction::North, Direction::East)]
    #[case(Direction::East, Direction::South)]
    #[case(Direction::South, Direction::West)]
    #[case(Direction::West, Direction::North)]
    fn test_turn_right(#[case] start_heading: Direction, #[case] expected_location: Direction) {
        let mut robot = PaintRobot::new();
        robot.heading = start_heading;
        robot.turn(1);
        assert_eq!(robot.heading, expected_location);
    }

    #[rstest]
    #[case(Direction::North, Direction::West, Point{ x: -1, y: 0})]
    #[case(Direction::East, Direction::North, Point{ x: 0, y: 1})]
    #[case(Direction::South, Direction::East, Point{ x: 1, y: 0})]
    #[case(Direction::West, Direction::South, Point{ x: 0, y: -1})]
    fn turn_left_and_move(
        #[case] start_heading: Direction,
        #[case] expected_heading: Direction,
        #[case] expected_location: Point<isize>,
    ) {
        let mut robot = PaintRobot::new();
        robot.heading = start_heading;
        robot.turn(0);
        robot.move_robot();
        assert_eq!(robot.heading, expected_heading);
        assert_eq!(robot.location, expected_location);
    }

    #[rstest]
    #[case(Direction::North, Direction::East, Point{ x: 1, y: 0})]
    #[case(Direction::East, Direction::South, Point{ x: 0, y: -1})]
    #[case(Direction::South, Direction::West, Point{ x: -1, y: 0})]
    #[case(Direction::West, Direction::North, Point{ x: 0, y: 1})]
    fn turn_right_and_move(
        #[case] start_heading: Direction,
        #[case] expected_heading: Direction,
        #[case] expected_location: Point<isize>,
    ) {
        let mut robot = PaintRobot::new();
        robot.heading = start_heading;
        robot.turn(1);
        robot.move_robot();
        assert_eq!(robot.heading, expected_heading);
        assert_eq!(robot.location, expected_location);
    }
}
