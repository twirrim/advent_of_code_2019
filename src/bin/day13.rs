use std::cmp::{max, min};

use image::{imageops, ImageBuffer, RgbImage};
use log::info;
use simple_logger::SimpleLogger;

use advent_of_code_2019::vm::VM;
use advent_of_code_2019::{debug_println, read_file, Point};

#[derive(Debug)]
enum Tile {
    Empty,
    Wall,
    Block,
    HPaddle,
    Ball,
}

impl Tile {
    fn from(number: isize) -> Self {
        match number {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HPaddle,
            4 => Tile::Ball,
            _ => panic!("Unknown tile type: {number}"),
        }
    }
}

#[derive(Debug)]
struct Location {
    point: Point<isize>,
    tile: Tile,
}

fn make_image_of_board(source: &Vec<Location>, name: &str) {
    let image_start = std::time::Instant::now();
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for loc in source {
        min_x = min(min_x, loc.point.x);
        min_y = min(min_y, loc.point.y);
        max_x = max(max_x, loc.point.x);
        max_y = max(max_y, loc.point.y);
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
    for loc in source {
        let pixel = img.get_pixel_mut(
            (loc.point.x + offset_x) as u32,
            (loc.point.y + offset_y) as u32,
        );
        match loc.tile {
            Tile::Empty => *pixel = image::Rgb([0, 0, 0]),
            Tile::Wall => *pixel = image::Rgb([255, 255, 255]),
            Tile::Block => *pixel = image::Rgb([255, 0, 0]),
            Tile::HPaddle => *pixel = image::Rgb([0, 255, 0]),
            Tile::Ball => *pixel = image::Rgb([0, 0, 255]),
        }
    }
    // Scale image
    let scaled_img = imageops::resize(
        &img,
        ((max_x + offset_x + 1) * 16) as u32,
        ((max_y + offset_y + 1) * 16) as u32,
        imageops::FilterType::Nearest,
    );
    info!("Saving image {name}");
    scaled_img.save(name).unwrap();
    info!("{} image creation took: {:?}", name, image_start.elapsed());
}

fn part_one(program: &[isize]) {
    let mut vm = VM::new(program.to_owned());
    vm.run();
    // This makes the assumption based on the problem description that output will be in 3s.
    let mut tile_count = 0;
    while vm.has_output() {
        let _x = match vm.pop_front_output() {
            Some(x) => x,
            None => panic!("Expected to get an x value"),
        };
        let _y = match vm.pop_front_output() {
            Some(x) => x,
            None => panic!("Expected to get an y value"),
        };
        let tile = match vm.pop_front_output() {
            Some(x) => x,
            None => panic!("Expected to get a Tile value"),
        };

        if tile == 2 {
            tile_count += 1;
        }
    }
    info!("Part one: {tile_count}");
}

fn part_two(program: &[isize]) {
    let mut vm = VM::new(program.to_owned());
    vm.run();
    let mut board: Vec<Location> = vec![];
    // This makes the assumption based on the problem description that output will be in 3s.
    while vm.has_output() {
        let x = match vm.pop_front_output() {
            Some(x) => x,
            None => panic!("Expected to get an x value"),
        };
        let y = match vm.pop_front_output() {
            Some(x) => x,
            None => panic!("Expected to get an y value"),
        };
        let tile: Tile = match vm.pop_front_output() {
            Some(x) => Tile::from(x),
            None => panic!("Expected to get a Tile value"),
        };
        board.push(Location {
            point: Point { x, y },
            tile,
        })
    }
    debug_println!("{:?}", board);
    make_image_of_board(&board, "part_two.png");
}

fn main() {
    let start = std::time::Instant::now();
    SimpleLogger::new().env().init().unwrap();
    info!("Reading input");
    // Only a single line in the input
    let input = read_file("./input/day13")[0]
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
