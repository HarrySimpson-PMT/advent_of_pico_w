use tokio::io;

use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 8, Part A");

    let mut positions: HashMap<char, Vec<Position>> = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch != '.' {
                positions
                    .entry(ch)
                    .or_insert_with(Vec::new)
                    .push(Position {
                        x: x as isize,
                        y: y as isize,
                    });
            }
        }
    }

    let grid_width = lines[0].len() as isize;
    let grid_height = lines.len() as isize;

    let mut found_points: HashSet<Position> = HashSet::new();

    for (&key, points) in &positions {
        println!("Processing key '{}'", key);
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let p1 = points[i];
                let p2 = points[j];

                let dx = p2.x - p1.x;
                let dy = p2.y - p1.y;

                let antinode1 = Position {
                    x: p1.x - dx,
                    y: p1.y - dy,
                };
                let antinode2 = Position {
                    x: p2.x + dx,
                    y: p2.y + dy,
                };

                println!("Node1: {:?}, Node2: {:?}, Antinode1: {:?}, Antinode2: {:?}", p1, p2, antinode1, antinode2);

                if is_within_bounds(antinode1, grid_width, grid_height) {
                    found_points.insert(antinode1);
                }
                if is_within_bounds(antinode2, grid_width, grid_height) {
                    found_points.insert(antinode2);
                }
            }
        }
    }
    let mut count = 0;
    for y in 0..grid_height {
        for x in 0..grid_width {
            let pos = Position { x, y };
            if found_points.contains(&pos) {
                print!("*");
                count += 1;
            } else {
                print!("{}", lines[y as usize].chars().nth(x as usize).unwrap());
                if lines[y as usize].chars().nth(x as usize).unwrap() != '.' {
                    count += 1;
                }
            }
        }
        println!();
    }
    
    print!("Number of unique found points: {}", count);

    println!("Number of unique found points: {}", found_points.len());

    Ok(())
}

fn is_within_bounds(pos: Position, width: isize, height: isize) -> bool {
    pos.x >= 0 && pos.x < width && pos.y >= 0 && pos.y < height
}

pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 8, Part B");

    let mut positions: HashMap<char, Vec<Position>> = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch != '.' {
                positions
                    .entry(ch)
                    .or_insert_with(Vec::new)
                    .push(Position {
                        x: x as isize,
                        y: y as isize,
                    });
            }
        }
    }

    let grid_width = lines[0].len() as isize;
    let grid_height = lines.len() as isize;

    let mut found_points: HashSet<Position> = HashSet::new();

    for (&key, points) in &positions {
        println!("Processing key '{}'", key);
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let p1 = points[i];
                let p2 = points[j];

                let dx = p2.x - p1.x;
                let dy = p2.y - p1.y;

                let mut antinode1 = Position { x: p1.x - dx, y: p1.y - dy };
                let mut antinode2 = Position { x: p2.x + dx, y: p2.y + dy };

                while is_within_bounds(antinode1, grid_width, grid_height) {
                    found_points.insert(antinode1);
                    antinode1 = Position {
                        x: antinode1.x - dx,
                        y: antinode1.y - dy,
                    };
                }

                while is_within_bounds(antinode2, grid_width, grid_height) {
                    found_points.insert(antinode2);
                    antinode2 = Position {
                        x: antinode2.x + dx,
                        y: antinode2.y + dy,
                    };
                }

                println!(
                    "Node1: {:?}, Node2: {:?}, Last Antinode1: {:?}, Last Antinode2: {:?}",
                    p1, p2, antinode1, antinode2
                );
            }
        }
    }

    let mut count = 0;
    for y in 0..grid_height {
        for x in 0..grid_width {
            let pos = Position { x, y };
            if found_points.contains(&pos) {
                print!("*");
                count += 1;
            } else {
                print!("{}", lines[y as usize].chars().nth(x as usize).unwrap());
                if lines[y as usize].chars().nth(x as usize).unwrap() != '.' {
                    count += 1;
                }
            }
        }
        println!();
    }
    
    println!("Number of unique found points: {}", count);

    println!("Number of unique found points: {}", found_points.len());

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input_for_puzzle;
    use crate::Puzzle;
    /// Determines the day name (e.g., "Day01") based on the module path
    fn get_day_name() -> String {
        let module_path = module_path!(); // e.g., "puzzles::day01"
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
