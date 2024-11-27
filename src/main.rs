#[allow(dead_code)]

use std::fs;
use std::path::Path;
mod puzzles;

use puzzles::*;

#[derive(Debug)]
enum Puzzle {
    Day01A,
    Day01B,
    Day02A,
    Day02B,
    Day03A,
    Day03B,
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
            
            // Add other days here
            _ => panic!("Invalid day or part"),
        }
    }
}

fn main() {
    // Select the puzzle you want to run
    let selected_puzzle = Puzzle::Day03A;

    // Load the corresponding input file
    if let Some(input_lines) = get_input_for_puzzle(&selected_puzzle) {
        match selected_puzzle {
            Puzzle::Day01A => day01::solve_a(&input_lines),
            Puzzle::Day01B => day01::solve_b(&input_lines),
            Puzzle::Day02A => day02::solve_a(&input_lines),
            Puzzle::Day02B => day02::solve_b(&input_lines),
            Puzzle::Day03A => day03::solve_a(&input_lines),
            Puzzle::Day03B => day03::solve_b(&input_lines),
        }
    } else {
        println!("Input file not found for puzzle: {:?}", selected_puzzle);
    }
}


/// Retrieves the input file for the selected puzzle
fn get_input_for_puzzle(puzzle: &Puzzle) -> Option<Vec<String>> {
    let file_name = match puzzle {
        Puzzle::Day01A => "day01_a.txt",
        Puzzle::Day01B => "day01_b.txt",
        Puzzle::Day02A => "day02_a.txt",
        Puzzle::Day02B => "day02_b.txt",
        Puzzle::Day03A => "day03_a.txt",
        Puzzle::Day03B => "day03_b.txt",
    };

    let input_path = Path::new("inputs").join(file_name);
    match fs::read_to_string(input_path) {
        Ok(content) => Some(content.lines().map(String::from).collect()),
        Err(_) => None,
    }
}

