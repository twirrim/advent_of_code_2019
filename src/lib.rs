use std::fs;
use std::io::prelude::*;

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
#[must_use]
#[inline]
pub fn read_file(source: &str) -> Vec<String> {
    // Reads in provided filename and returns a Vec<String>
    fs::read_to_string(source)
        .expect("Unable to read file")
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}
