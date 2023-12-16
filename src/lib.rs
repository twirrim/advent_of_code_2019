use std::fs;
use std::io;
use std::io::prelude::*;

use log::debug;

pub mod vm;

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
pub fn get_user_input(query: String) -> String {
    let mut input = String::new();
    println!("{query}");
    io::stdin().read_line(&mut input).unwrap();
    input
}
