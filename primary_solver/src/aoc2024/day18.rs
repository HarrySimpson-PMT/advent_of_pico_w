use std::collections::{HashSet, VecDeque};
use tokio::io;

#[derive(Debug)]
struct Grid {
    size: usize,
    grid: Vec<Vec<bool>>,
}

impl Grid {
    fn new(size: usize) -> Self {
        Grid {
            size,
            grid: vec![vec![false; size]; size],
        }
    }

    fn corrupt(&mut self, x: usize, y: usize) {
        if x < self.size && y < self.size {
            self.grid[y][x] = true;
        }
    }

    fn is_safe(&self, x: usize, y: usize) -> bool {
        x < self.size && y < self.size && !self.grid[y][x]
    }

    fn print(&self) {
        for row in self.grid.iter() {
            for &cell in row.iter() {
                print!("{}", if cell { '#' } else { '.' });
            }
            println!();
        }
    }

    fn find_shortest_path(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(start);
        visited.insert(start);

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        while let Some((x, y)) = queue.pop_front() {
            if (x, y) == end {
                return true;
            }

            for (dx, dy) in directions.iter() {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0
                    && ny >= 0
                    && self.is_safe(nx as usize, ny as usize)
                    && visited.insert((nx as usize, ny as usize))
                {
                    queue.push_back((nx as usize, ny as usize));
                }
            }
        }

        false
    }
}

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 18, Part A");

    let mut grid = Grid::new(71);

    let coordinates: Vec<(usize, usize)> = lines
        .iter()
        .map(|line| {
            let parts: Vec<usize> = line
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            (parts[0], parts[1])
        })
        .collect();

    for &coord in coordinates.iter().take(1024) {
        grid.corrupt(coord.0, coord.1);
    }

    grid.print();

    let start = (0, 0);
    let end = (70, 70);
    match grid.find_shortest_path(start, end) {
        true => println!("Path found"),
        false => println!("No path found"),
    }

    Ok(())
}


pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 18, Part B");

    let mut grid = Grid::new(71); 

    let coordinates: Vec<(usize, usize)> = lines
        .iter()
        .map(|line| {
            let parts: Vec<usize> = line
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            (parts[0], parts[1])
        })
        .collect();

    let start = (0, 0);
    let end = (70, 70);

    for (i, &coord) in coordinates.iter().enumerate() {
        grid.corrupt(coord.0, coord.1);
        println!("Corrupted byte: {},{}", coord.0, coord.1);

        if !grid.find_shortest_path(start, end) {
            println!("First blocking byte: {},{}", coord.0, coord.1);
            break;
        }
    }

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
