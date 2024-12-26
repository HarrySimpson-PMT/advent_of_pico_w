use tokio::io;
use std::error::Error;

pub async fn solve_a(lines: &Vec<String>) -> Result<(), Box<dyn Error>> {
    println!("Solving Day 25, Part A");

    let schematics = split_schematics(lines)?;

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for schematic in schematics {
        if schematic.len() != 7 {
            eprintln!(
                "Warning: Skipping schematic with {} lines (expected 7).",
                schematic.len()
            );
            continue;
        }

        let top_row = &schematic[0];
        let is_lock = top_row.chars().all(|c| c == '#');

        let heights = if is_lock {
            parse_lock_heights(&schematic)
        } else {
            parse_key_heights(&schematic)
        };

        if is_lock {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    let mut valid_pairs = 0;
    for lock in &locks {
        for key in &keys {
            if fits(lock, key) {
                valid_pairs += 1;
            }
        }
    }

    println!(
        "Number of unique lock/key pairs that fit: {}",
        valid_pairs
    );

    Ok(())
}

fn split_schematics(lines: &Vec<String>) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut schematics = Vec::new();
    let mut current = Vec::new();

    for line in lines {
        if line.trim().is_empty() {
            if !current.is_empty() {
                schematics.push(current.clone());
                current.clear();
            }
        } else {
            current.push(line.clone());
        }
    }

    if !current.is_empty() {
        schematics.push(current);
    }

    Ok(schematics)
}

fn parse_lock_heights(schematic: &[String]) -> Vec<usize> {
    let mut heights = vec![0; 5];
    for col in 0..5 {
        let mut height = 0;
        for row in 1..6 {
            let c = schematic[row].chars().nth(col).unwrap_or('.');
            if c == '#' {
                height += 1;
            } else {
                break;
            }
        }
        heights[col] = height;
    }
    heights
}

fn parse_key_heights(schematic: &[String]) -> Vec<usize> {
    let mut heights = vec![0; 5];
    for col in 0..5 {
        let mut height = 0;
        for row in (1..6).rev() {
            let c = schematic[row].chars().nth(col).unwrap_or('.');
            if c == '#' {
                height += 1;
            } else {
                break;
            }
        }
        heights[col] = height;
    }
    heights
}

fn fits(lock: &Vec<usize>, key: &Vec<usize>) -> bool {
    for i in 0..5 {
        if lock[i] + key[i] > 5 {
            return false;
        }
    }
    true
}

pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 6, Part B");
    for line in lines {
        println!("{}", line);
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input_for_puzzle;
    use crate::Puzzle;
    fn get_day_name() -> String {
        let module_path = module_path!();
        let module_name = module_path.split("::").last().unwrap_or("Unknown");
        module_name.to_string().replace("day", "Day")
    }

    fn get_puzzle(part: char) -> Puzzle {
        let day = get_day_name().replace("Day", "").parse::<u8>().unwrap_or(1);
        match part {
            'A' => Puzzle::from_day_part(day, 'A'),
            'B' => Puzzle::from_day_part(day, 'B'),
            _ => panic!("Invalid part"),
        }
    }

    // #[test]
    // fn test_solve_a_with_real_input() {
    //     let puzzle = get_puzzle('A');
    //     if let Some(input) = get_input_for_puzzle(&puzzle) {
    //         solve_a(&input);
    //         assert!(true, "Add your assertions here");
    //     } else {
    //         panic!("Input file not found for {:?}", puzzle);
    //     }
    // }

    // #[test]
    // #[ignore]
    // fn test_solve_b_with_real_input() {
    //     let puzzle = get_puzzle('B');
    //     if let Some(input) = get_input_for_puzzle(&puzzle) {
    //         solve_b(&input);
    //         assert!(true, "Add your assertions here");
    //     } else {
    //         panic!("Input file not found for {:?}", puzzle);
    //     }
    // }
}
