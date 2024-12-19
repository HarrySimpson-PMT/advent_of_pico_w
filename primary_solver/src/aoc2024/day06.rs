use std::collections::HashSet;
use tokio::io;

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 6, Part A");

    let mut grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let mut start_pos = (0, 0);
    for (y, row) in grid.iter().enumerate() {
        if let Some(x) = row.iter().position(|&c| c == '^') {
            start_pos = (x, y);
            break;
        }
    }

    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut dir_index = 0; 

    let mut visited = HashSet::new();
    let (mut x, mut y) = start_pos;

    while y < grid.len() && x < grid[0].len() {
        visited.insert((x, y));
        grid[y][x] = 'X';

        let (dx, dy) = directions[dir_index];
        let (next_x, next_y) = ((x as isize + dx) as usize, (y as isize + dy) as usize);

        if next_x >= grid[0].len() || next_y >= grid.len() {
            break;
        }

        if grid[next_y][next_x] == '#' {
            dir_index = (dir_index + 1) % 4;
        } else {
            x = next_x;
            y = next_y;
        }
    }
    println!();
    println!("Final location: ({}, {})", x, y);
    for row in &grid {
        println!("{}", row.iter().collect::<String>());
    }

    println!("Visited locations: {}", visited.len());
    Ok(())
}

pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 6, Part B");

    let mut grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let mut start_pos = (0, 0);
    for (y, row) in grid.iter().enumerate() {
        if let Some(x) = row.iter().position(|&c| c == '^') {
            start_pos = (x, y);
            grid[y][x] = 'X';
            break;
        }
    }

    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    #[derive(Clone, Copy, PartialEq, Eq)]
    struct Position {
        x: usize,
        y: usize,
        dir_index: usize,
    }

    use std::collections::HashSet;

    fn loop_check(
        grid: &Vec<Vec<char>>,
        mut pos: Position,
        directions: &[(isize, isize)],
    ) -> bool {
        let mut visited_positions = HashSet::new();
    
        loop {
            let (dx, dy) = directions[pos.dir_index];
            let next_x = pos.x as isize + dx;
            let next_y = pos.y as isize + dy;
    
            if next_x < 0
                || next_y < 0
                || next_x >= grid[0].len() as isize
                || next_y >= grid.len() as isize
            {
                return false;
            }
    
            let (nx, ny) = (next_x as usize, next_y as usize);
    
            if visited_positions.contains(&(nx, ny, pos.dir_index)) {
                return true;
            }
    
            visited_positions.insert((nx, ny, pos.dir_index));
    
            if grid[ny][nx] == '#' {
                pos.dir_index = (pos.dir_index + 1) % 4; // Turn 90° right
            } else {
                pos.x = nx;
                pos.y = ny;
            }
        }
    }

    let mut result = 0;
    let mut pos = Position {
        x: start_pos.0,
        y: start_pos.1,
        dir_index: 0,
    };

    loop {
        grid[pos.y][pos.x] = 'X';

        let (dx, dy) = directions[pos.dir_index];
        let next_x = pos.x as isize + dx;
        let next_y = pos.y as isize + dy;

        if next_x < 0
            || next_y < 0
            || next_x >= grid[0].len() as isize
            || next_y >= grid.len() as isize
        {
            break;
        }

        let (nx, ny) = (next_x as usize, next_y as usize);

        if grid[ny][nx] == '.' {
            grid[ny][nx] = '#'; 
            if loop_check(
                &grid,
                Position {
                    x: pos.x,
                    y: pos.y,
                    dir_index: pos.dir_index,
                },
                &directions,
            ) {
                result += 1;
            }
            grid[ny][nx] = '.';
        }

        if grid[ny][nx] == '#' {
            pos.dir_index = (pos.dir_index + 1) % 4; // Turn 90° right
        } else {
            pos.x = nx;
            pos.y = ny;
        }
    }

    println!("Number of loop-inducing positions: {}", result);
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
