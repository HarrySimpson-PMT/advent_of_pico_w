use std::collections::{HashMap, HashSet, VecDeque};
use tokio::io;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn to_offset(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }
}

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 16, Part A");

    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let start = find_position(&grid, 'S');
    let end = find_position(&grid, 'E');

    let mut queue = VecDeque::new();
    let mut costs = HashMap::new();

    let initial_direction = Direction::East;
    queue.push_back((start, initial_direction, 0));
    costs.insert((start, initial_direction), 0);

    let mut min_cost = usize::MAX;

    while let Some(((x, y), direction, cost)) = queue.pop_front() {
        if (x, y) == end {
            min_cost = min_cost.min(cost);
            continue;
        }

        // Move forward
        let (dx, dy) = direction.to_offset();
        let next_pos = ((x as isize + dx) as usize, (y as isize + dy) as usize);
        if is_valid(&grid, next_pos) {
            let next_cost = cost + 1;
            if next_cost < *costs.get(&(next_pos, direction)).unwrap_or(&usize::MAX) {
                costs.insert((next_pos, direction), next_cost);
                queue.push_back((next_pos, direction, next_cost));
            }
        }

        // Turn left
        let left_dir = direction.turn_left();
        let left_cost = cost + 1000;
        if left_cost < *costs.get(&((x, y), left_dir)).unwrap_or(&usize::MAX) {
            costs.insert(((x, y), left_dir), left_cost);
            queue.push_back(((x, y), left_dir, left_cost));
        }

        // Turn right
        let right_dir = direction.turn_right();
        let right_cost = cost + 1000;
        if right_cost < *costs.get(&((x, y), right_dir)).unwrap_or(&usize::MAX) {
            costs.insert(((x, y), right_dir), right_cost);
            queue.push_back(((x, y), right_dir, right_cost));
        }
    }

    println!("Minimum cost: {}", min_cost);

    Ok(())
}

fn find_position(grid: &Vec<Vec<char>>, target: char) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == target {
                return (i, j);
            }
        }
    }
    panic!("Target not found in grid");
}

fn is_valid(grid: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    let (x, y) = pos;
    x < grid.len() && y < grid[0].len() && grid[x][y] != '#'
}
pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 16, Part A");

    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let start = find_position(&grid, 'S');
    let end = find_position(&grid, 'E');

    let mut queue = VecDeque::new();
    let mut costs = HashMap::new();
    let mut parents: HashMap<((usize, usize), Direction), Vec<(usize, usize, Direction)>> =
        HashMap::new();

    let initial_direction = Direction::East;
    queue.push_back((start, initial_direction, 0)); // (position, direction, cost)
    costs.insert((start, initial_direction), 0);

    let mut min_cost = usize::MAX;
    let mut end_states = Vec::new();

    while let Some(((x, y), direction, cost)) = queue.pop_front() {
        if (x, y) == end {
            if cost < min_cost {
                min_cost = cost;
                end_states.clear();
            }
            if cost == min_cost {
                end_states.push(((x, y), direction));
            }
            continue;
        }

        // Move forward
        let (dx, dy) = direction.to_offset();
        let next_pos = ((x as isize + dx) as usize, (y as isize + dy) as usize);
        if is_valid(&grid, next_pos) {
            let next_cost = cost + 1;
            if next_cost < *costs.get(&(next_pos, direction)).unwrap_or(&usize::MAX) {
                costs.insert((next_pos, direction), next_cost);
                if parents.contains_key(&(next_pos, direction)) {
                    parents.remove(&(next_pos, direction));
                    parents.insert((next_pos, direction), vec![(x, y, direction)]);
                } else {
                    parents.insert((next_pos, direction), vec![(x, y, direction)]);
                }
                queue.push_back((next_pos, direction, next_cost));
            }
            else if next_cost == *costs.get(&(next_pos, direction)).unwrap_or(&usize::MAX) {
                if parents.contains_key(&(next_pos, direction)) {
                    let parents_vec = parents.get_mut(&(next_pos, direction)).unwrap();
                    parents_vec.push((x, y, direction));
                } else {
                    parents.insert((next_pos, direction), vec![(x, y, direction)]);
                }
            }
        }

        // Turn left
        let left_dir = direction.turn_left();
        let left_cost = cost + 1000;
        if left_cost < *costs.get(&((x, y), left_dir)).unwrap_or(&usize::MAX) {
            costs.insert(((x, y), left_dir), left_cost);
            if parents.contains_key(&((x, y), left_dir)) {
                parents.remove(&((x, y), left_dir));
            }
            parents.insert(((x, y), left_dir), vec![(x, y, direction)]);
            queue.push_back(((x, y), left_dir, left_cost));
        } else if left_cost == *costs.get(&((x, y), left_dir)).unwrap_or(&usize::MAX) {
            if parents.contains_key(&((x, y), left_dir)) {
                let parents_vec = parents.get_mut(&((x, y), left_dir)).unwrap();
                parents_vec.push((x, y, direction));
            } else {
                parents.insert(((x, y), left_dir), vec![(x, y, direction)]);
            }
        }

        // Turn right
        let right_dir = direction.turn_right();
        let right_cost = cost + 1000;
        if right_cost < *costs.get(&((x, y), right_dir)).unwrap_or(&usize::MAX) {
            costs.insert(((x, y), right_dir), right_cost);
            if parents.contains_key(&((x, y), right_dir)) {
                parents.remove(&((x, y), right_dir));
            }
            parents.insert(((x, y), right_dir), vec![(x, y, direction)]);
            queue.push_back(((x, y), right_dir, right_cost));
        } else if right_cost == *costs.get(&((x, y), right_dir)).unwrap_or(&usize::MAX) {
            if parents.contains_key(&((x, y), right_dir)) {
                 parents.get_mut(&((x, y), right_dir)).unwrap().push((x, y, direction));
            } else {
                parents.insert(((x, y), right_dir), vec![(x, y, direction)]);
            }
        }
    }


    println!("Backtracking optimal paths");
    let mut path = HashSet::new();
    let mut visited = HashSet::new();
    for (end_pos, end_dir) in end_states {
        let mut stack = VecDeque::new();
        stack.push_back((end_pos, end_dir));
        while let Some((pos, dir)) = stack.pop_back() {
            path.insert(pos);
            if let Some(parents_vec) = parents.get(&(pos, dir)) {
                for (parent_x, parent_y, parent_dir) in parents_vec {
                    if visited.contains(&((*parent_x, *parent_y), *parent_dir)) {
                        continue;
                    }
                    visited.insert(((*parent_x, *parent_y), *parent_dir));
                    stack.push_back(((*parent_x, *parent_y), *parent_dir));
                }
            }
        }
    }

    let mut grid_with_path = grid.clone();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if path.contains(&(i, j)) {
                grid_with_path[i][j] = 'O';
            }
        }
    }

    for row in grid_with_path {
        println!("{}", row.iter().collect::<String>());
    }

    println!("Number of tiles in optimal paths: {}", path.len());

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
