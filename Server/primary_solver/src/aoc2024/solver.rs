#[allow(dead_code)]
use std::fs;
use std::path::Path;


#[derive(Debug)]
enum Puzzle {
    Day01A,
    Day01B,
    Day02A,
    Day02B,
    Day03A,
    Day03B,
    Day04A,
    Day04B,
}

impl Puzzle {
    #[allow(dead_code)]
    pub fn from_day_part(day: u8, part: char) -> Self {
        match (day, part) {
            (1, 'A') => Puzzle::Day01A,
            (1, 'B') => Puzzle::Day01B,
            (2, 'A') => Puzzle::Day02A,
            (2, 'B') => Puzzle::Day02B,
            (3, 'A') => Puzzle::Day03A,
            (3, 'B') => Puzzle::Day03B,
            (4, 'A') => Puzzle::Day04A,
            (4, 'B') => Puzzle::Day04B,       

            // Add other days here
            _ => panic!("Invalid day or part"),
        }
    }

    // Helper method to extract (day, part)
    pub fn to_day_part(&self) -> (u8, char) {
        match self {
            Puzzle::Day01A => (1, 'A'),
            Puzzle::Day01B => (1, 'B'),
            Puzzle::Day02A => (2, 'A'),
            Puzzle::Day02B => (2, 'B'),
            Puzzle::Day03A => (3, 'A'),
            Puzzle::Day03B => (3, 'B'),
            Puzzle::Day04A => (4, 'A'),
            Puzzle::Day04B => (4, 'B'),
        }
    }
}
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
