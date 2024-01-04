//! This library provides a function to read a line from the standard input and return it as a String.
//! # Examples:
//! ```
//! use read_stdin::read_stdin;
//! let input = read_stdin();
//! ```
//! # Panics:
//! This function will panic if it fails to read a line from the standard input.
//! # Errors:
//! This function will return an error if it fails to read a line from the standard input.
//! # Safety:
//! This function is safe to use.
//! # Notes:
//! This function will return an empty String if the user enters an empty line.
//! # Examples:
//! ```
//! use read_stdin::read_stdin;
//! let input = read_stdin();
//! ```
use std::io::{BufRead, BufReader};

pub mod colors;
pub mod config;

/// This function reads a line from the standard input and returns it as a String.
/// It will panic if it fails to read a line.

pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut buffer = String::new();
    reader.read_line(&mut buffer).expect("Failed to read line");
    buffer.trim().to_string()
}
