#[allow(dead_code)]
use std::fs;
use std::path::Path;
mod aoc2024;
mod comms;

use aoc2024::*;
use comms::pico_sender::send_data_to_pico;

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
    Day05A,
    Day05B,
    Day06A,
    Day06B,
    Day07A,
    Day07B,
    Day08A,
    Day08B,
    Day09A,
    Day09B,
    Day10A,
    Day10B,
    Day11A,
    Day11B,
    Day12A,
    Day12B,
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
            (5, 'A') => Puzzle::Day05A,
            (5, 'B') => Puzzle::Day05B,
            (6, 'A') => Puzzle::Day06A,
            (6, 'B') => Puzzle::Day06B,
            (7, 'A') => Puzzle::Day07A,
            (7, 'B') => Puzzle::Day07B,
            (8, 'A') => Puzzle::Day08A,
            (8, 'B') => Puzzle::Day08B,
            (9, 'A') => Puzzle::Day09A,
            (9, 'B') => Puzzle::Day09B,
            (10, 'A') => Puzzle::Day10A,
            (10, 'B') => Puzzle::Day10B,
            (11, 'A') => Puzzle::Day11A,
            (11, 'B') => Puzzle::Day11B,
            (12, 'A') => Puzzle::Day12A,
            (12, 'B') => Puzzle::Day12B,
            _ => panic!("Invalid day or part"),
        }
    }

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
            Puzzle::Day05A => (5, 'A'),
            Puzzle::Day05B => (5, 'B'),
            Puzzle::Day06A => (6, 'A'),
            Puzzle::Day06B => (6, 'B'),
            Puzzle::Day07A => (7, 'A'),
            Puzzle::Day07B => (7, 'B'),
            Puzzle::Day08A => (8, 'A'),
            Puzzle::Day08B => (8, 'B'),
            Puzzle::Day09A => (9, 'A'),
            Puzzle::Day09B => (9, 'B'),
            Puzzle::Day10A => (10, 'A'),
            Puzzle::Day10B => (10, 'B'),
            Puzzle::Day11A => (11, 'A'),
            Puzzle::Day11B => (11, 'B'),
            Puzzle::Day12A => (12, 'A'),
            Puzzle::Day12B => (12, 'B'),
        }
    }
}

#[tokio::main]
async fn main() {
    let selected_puzzle = Puzzle::Day01A;
    let somelines = match get_input_for_puzzle(&selected_puzzle) {
        Some(lines) => lines,
        None => {
            println!("Input file not found for puzzle: {:?}", selected_puzzle);
            return;
        }
    };
    send_data_to_pico(&somelines).await;

    return;

    let selected_puzzle = Puzzle::Day11B;


    if let Some(input_lines) = get_input_for_puzzle(&selected_puzzle) {
        println!("Number of lines: {}", input_lines.len());

        match selected_puzzle {
            Puzzle::Day01A => {
                if let Err(e) = day01::solve_a(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day01B => {
                if let Err(e) = day01::solve_b(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day02A => {
                if let Err(e) = day02::solve_a(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day02B => {
                if let Err(e) = day02::solve_b(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day03A => {
                if let Err(e) = day03::solve_a(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day03B => {
                if let Err(e) = day03::solve_b(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day04A => {
                if let Err(e) = day04::solve_a(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day04B => {
                if let Err(e) = day04::solve_b(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day05A => {
                if let Err(e) = day05::solve_a(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day05B => {
                if let Err(e) = day05::solve_b(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day06A => {
                if let Err(e) = day06::solve_a(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day06B => {
                if let Err(e) = day06::solve_b(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day07A => {
                if let Err(e) = day07::solve_a(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day07B => {
                if let Err(e) = day07::solve_b(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day08A => {
                if let Err(e) = day08::solve_a(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day08B => {
                if let Err(e) = day08::solve_b(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day09A => {
                if let Err(e) = day09::solve_a(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day09B => {
                if let Err(e) = day09::solve_b(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day10A => {
                if let Err(e) = day10::solve_a(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day10B => {
                if let Err(e) = day10::solve_b(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day11A => {
                if let Err(e) = day11::solve_a(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day11B => {
                if let Err(e) = day11::solve_b(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day12A => {
                if let Err(e) = day12::solve_a(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }   
            Puzzle::Day12B => {
                if let Err(e) = day12::solve_b(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
        }
    } else {
        println!("Input file not found for puzzle: {:?}", selected_puzzle);
    }
}

fn get_input_for_puzzle(puzzle: &Puzzle) -> Option<Vec<String>> {
    let (day, _part) = puzzle.to_day_part();

    let file_name = format!("day{:02}//file.txt", day);
    let input_path = Path::new("primary_solver\\inputs\\2024").join(file_name);
    print!("{:?}", input_path);

    fs::read_to_string(input_path)
        .ok()
        .map(|content| content.lines().map(String::from).collect())
}



