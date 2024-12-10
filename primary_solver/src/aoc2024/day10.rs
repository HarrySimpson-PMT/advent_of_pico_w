use tokio::io;

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    // Parse the grid from input lines
    let grid: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    // Dimensions of the grid
    let rows = grid.len();
    let cols = grid[0].len();

    // Function to calculate the score (number of paths to 9) from a given trailhead (x, y)
    fn count_paths_to_peak(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> usize {
        let mut stack = vec![(x, y)];
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let mut score = 0;

        while let Some((cx, cy)) = stack.pop() {
            if visited[cx][cy] {
                continue;
            }
            visited[cx][cy] = true;

            if grid[cx][cy] == 9 {
                score += 1;
                continue;
            }

            // Check cardinal directions
            for (nx, ny) in [
                (cx.wrapping_sub(1), cy), // Up
                (cx + 1, cy),             // Down
                (cx, cy.wrapping_sub(1)), // Left
                (cx, cy + 1),             // Right
            ] {
                if nx < grid.len()
                    && ny < grid[0].len()
                    && grid[nx][ny] == grid[cx][cy] + 1
                {
                    stack.push((nx, ny));
                }
            }
        }

        score
    }

    // Sum the scores for all trailheads (cells with 0)
    let mut total_score = 0;
    for x in 0..rows {
        for y in 0..cols {
            if grid[x][y] == 0 {
                total_score += count_paths_to_peak(&grid, x, y);
            }
        }
    }

    println!("Total Score: {}", total_score);

    Ok(())
}
use std::collections::HashSet;


pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    // Parse the grid from input lines
    let grid: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    // Dimensions of the grid
    let rows = grid.len();
    let cols = grid[0].len();

    // Recursive DFS to find distinct paths
    fn find_distinct_paths(
        grid: &Vec<Vec<u32>>,
        x: usize,
        y: usize,
        path: &mut Vec<(usize, usize)>,
        paths: &mut HashSet<Vec<(usize, usize)>>
    ) {
        if grid[x][y] == 9 {
            // Reached a peak, add path to set
            paths.insert(path.clone());
            return;
        }

        // Explore cardinal directions
        for (nx, ny) in [
            (x.wrapping_sub(1), y), // Up
            (x + 1, y),             // Down
            (x, y.wrapping_sub(1)), // Left
            (x, y + 1),             // Right
        ] {
            if nx < grid.len()
                && ny < grid[0].len()
                && grid[nx][ny] == grid[x][y] + 1
                && !path.contains(&(nx, ny)) // Avoid cycles
            {
                path.push((nx, ny)); // Add step to path
                find_distinct_paths(grid, nx, ny, path, paths);
                path.pop(); // Backtrack
            }
        }
    }

    // Sum the number of distinct paths for each trailhead (cells with 0)
    let mut total_paths = 0;
    for x in 0..rows {
        for y in 0..cols {
            if grid[x][y] == 0 {
                let mut paths = HashSet::new();
                let mut path = vec![(x, y)];
                find_distinct_paths(&grid, x, y, &mut path, &mut paths);
                total_paths += paths.len();
                println!(
                    "Trailhead ({}, {}): {} distinct paths",
                    x, y, paths.len()
                );
            }
        }
    }

    println!("Total Distinct Paths: {}", total_paths);

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
