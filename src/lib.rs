use std::fmt::Display;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::ops::AddAssign;

use num_traits::int::PrimInt;

pub mod vm;

// from https://www.reddit.com/r/rust/comments/skmpnr/output_text_to_console_in_debug_mode_only/hvluai2/
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => (if ::std::cfg!(debug_assertions) { ::std::println!($($arg)*); })
}

#[must_use]
#[inline]
pub fn read_stdin() -> Vec<String> {
    // Reads in from stdin and returns a Vec<String>
    let stdin = std::io::stdin();
    stdin.lock().lines().map(Result::unwrap).collect()
}

/// # Panics
///
/// Will panic if it can't read the file
#[inline]
pub fn read_file(source: &str) -> Vec<String> {
    // Reads in provided filename and returns a Vec<String>
    fs::read_to_string(source)
        .expect("Unable to read file")
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

/// # Panics
///
/// Will panic if it has problems reading from stdin
pub fn get_user_input(query: &str) -> String {
    let mut input = String::new();
    println!("{query}");
    io::stdin().read_line(&mut input).unwrap();
    input
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Point<T: PrimInt + Display> {
    pub x: T,
    pub y: T,
}

impl<T: PrimInt + Display> AddAssign for Point<T> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn turn_left(&self) -> Direction {
        match &self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }

    pub fn turn_right(&self) -> Direction {
        match &self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}
