use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};
use tokio::io;

#[derive(Clone, Eq, PartialEq)]
struct State {
    position: (usize, usize),
    time: usize,
    cheat_used: bool,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.time.cmp(&self.time).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    let mut grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 'S' {
                start = (r, c);
                grid[r][c] = '.';
            } else if grid[r][c] == 'E' {
                end = (r, c);
                grid[r][c] = '.';
            }
        }
    }
    println!();
    for r in 0..rows {
        for c in 0..cols {
            print!("{}", grid[r][c]);
        }
        println!();
    }
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut queue = BinaryHeap::new();
    queue.push(State {
        position: start,
        time: 0,
        cheat_used: false,
    });
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
    let mut standard_time = 0;
    while let Some(State {
        position,
        time,
        cheat_used,
    }) = queue.pop()
    {
        if position == end {
            break;
        }
        if visited.contains_key(&position) {
            continue;
        }
        visited.insert(position, time);
        for (dr, dc) in directions.iter() {
            let new_position = (position.0 as i32 + dr, position.1 as i32 + dc);
            if new_position.0 >= 0
                && new_position.0 < rows as i32
                && new_position.1 >= 0
                && new_position.1 < cols as i32
            {
                let new_position = (new_position.0 as usize, new_position.1 as usize);
                if grid[new_position.0][new_position.1] == '.' {
                    queue.push(State {
                        position: new_position,
                        time: time + 1,
                        cheat_used,
                    });
                }
                if new_position == end {
                    standard_time = time + 1;
                    break;
                }
            }
        }
    }

    // Evaluate possible cheats
    let mut cheats = BinaryHeap::new();
    let mut unique_cheats = HashSet::new();

    for (&position, &time) in visited.iter() {
        for &(dr, dc) in directions.iter() {
            for cheat_steps in 1..=2 {
                let new_position = (
                    position.0 as i32 + dr * cheat_steps,
                    position.1 as i32 + dc * cheat_steps,
                );
                if new_position.0 >= 0
                    && new_position.0 < rows as i32
                    && new_position.1 >= 0
                    && new_position.1 < cols as i32
                {
                    let new_position = (new_position.0 as usize, new_position.1 as usize);
                    if grid[new_position.0][new_position.1] == '.' {
                        if let Some(&actual_time) = visited.get(&new_position) {
                            let cheat_time = time + cheat_steps as usize + 1; // Add 1 for transition
                            if cheat_time < actual_time {
                                let time_saved = actual_time.saturating_sub(cheat_time);
                                if unique_cheats.insert((position, new_position)) {
                                    cheats.push((time_saved + 1, position, new_position));
                                }
                            }
                        }
                    }
                    if new_position == end {
                        let cheat_time = time + cheat_steps as usize;
                        if cheat_time < standard_time {
                            let time_saved = standard_time.saturating_sub(cheat_time);
                            if unique_cheats.insert((position, new_position)) {
                                cheats.push((time_saved, position, new_position));
                            }
                        }
                    }
                }
            }
        }
    }

    let mut best_cheats_over_100 = 0;
    // Extract cheats in priority order
    while let Some((time_saved, start, end)) = cheats.pop() {
        if time_saved >= 100 {
            best_cheats_over_100 += 1;
            println!(
                "Cheat from {:?} to {:?} saves {} picoseconds",
                start, end, time_saved
            );
        }
    }

    let result = best_cheats_over_100;

    println!("{}", result);

    Ok(())
}

pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 6, Part B");
    let mut grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 'S' {
                start = (r, c);
                grid[r][c] = '.';
            } else if grid[r][c] == 'E' {
                end = (r, c);
                grid[r][c] = '.';
            }
        }
    }
    println!();
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut queue = BinaryHeap::new();
    queue.push(State {
        position: start,
        time: 0,
        cheat_used: false,
    });
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
    let mut standard_time = 0;
    while let Some(State {
        position,
        time,
        cheat_used,
    }) = queue.pop()
    {
        if position == end {
            break;
        }
        if visited.contains_key(&position) {
            continue;
        }
        visited.insert(position, time);
        for (dr, dc) in directions.iter() {
            let new_position = (position.0 as i32 + dr, position.1 as i32 + dc);
            if new_position.0 >= 0
                && new_position.0 < rows as i32
                && new_position.1 >= 0
                && new_position.1 < cols as i32
            {
                let new_position = (new_position.0 as usize, new_position.1 as usize);
                if grid[new_position.0][new_position.1] == '.' {
                    queue.push(State {
                        position: new_position,
                        time: time + 1,
                        cheat_used,
                    });
                }
                if new_position == end {
                    standard_time = time + 1;
                    visited.insert(new_position, standard_time);
                    break;
                }
            }
        }
    }

    println!("Standard time: {}", standard_time);

    let mut cheats = BinaryHeap::new();
    for (&position, &time) in visited.iter() {
        let mut queue = BinaryHeap::new();
        queue.push((Reverse(0), position, time));
        let mut bfs_visited: HashSet<(usize, usize)> = HashSet::new();
        bfs_visited.insert(position);
        while let Some((Reverse(steps), current_position, current_time)) = queue.pop() {
            if steps > 19 || current_time >= standard_time {
                continue;
            }

            for &(dr, dc) in directions.iter() {
                let new_position = (
                    current_position.0 as i32 + dr,
                    current_position.1 as i32 + dc,
                );

                if new_position.0 >= 0
                    && new_position.0 < rows as i32
                    && new_position.1 >= 0
                    && new_position.1 < cols as i32
                {
                    let new_position = (new_position.0 as usize, new_position.1 as usize);
                    let new_time = current_time + 1;
                    if bfs_visited.insert(new_position) {
                        if grid[new_position.0][new_position.1] == '.' || new_position == end {
                            if let Some(&actual_time) = visited.get(&new_position) {
                                if actual_time > new_time {
                                    let time_saved = actual_time - new_time;
                                    if time_saved >= 100 {
                                        cheats.push((time_saved, position, new_position));
                                    }
                                }
                            }
                        }
                        queue.push((Reverse(steps + 1), new_position, new_time));
                    }
                }
            }
        }
    }

    let mut cheat_savings: HashMap<usize, usize> = HashMap::new();
    while let Some((time_saved, _, _)) = cheats.pop() {
        *cheat_savings.entry(time_saved).or_insert(0) += 1;
    }

    // println!("\nCheat Summary:");
    let mut sorted_savings: Vec<_> = cheat_savings.iter().collect();
    sorted_savings.sort_by_key(|&(time_saved, _)| time_saved);

    let mut result = 0;

    for (_, count) in sorted_savings {
        result += *count;

    }

    println!("Result: {}", result);

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
