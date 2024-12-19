use tokio::io::{self};

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    let width = 101;
    let height = 103;
    let mid_x = width / 2;
    let mid_y = height / 2;

    let mut quadrant_counts = [0; 4]; 

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let p: Vec<i32> = parts[0][2..]
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let v: Vec<i32> = parts[1][2..]
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let (p_x, p_y) = (p[0], p[1]);
        let (v_x, v_y) = (v[0], v[1]);

        let x_new = (p_x + 100 * v_x).rem_euclid(width as i32);
        let y_new = (p_y + 100 * v_y).rem_euclid(height as i32);

        if x_new == mid_x as i32 || y_new == mid_y as i32 {
            continue;
        }

        if x_new < mid_x as i32 && y_new < mid_y as i32 {
            quadrant_counts[0] += 1; // Top-left
        } else if x_new >= mid_x as i32 && y_new < mid_y as i32 {
            quadrant_counts[1] += 1; // Top-right
        } else if x_new < mid_x as i32 && y_new >= mid_y as i32 {
            quadrant_counts[2] += 1; // Bottom-left
        } else if x_new >= mid_x as i32 && y_new >= mid_y as i32 {
            quadrant_counts[3] += 1; // Bottom-right
        }
    }

    let safety_factor: i32 = quadrant_counts.iter().product();
    println!("Safety Factor: {}", safety_factor);

    Ok(())
}

pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    let width = 101;
    let height = 103;

    let mut robots: Vec<((i32, i32), (i32, i32))> = lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let p: Vec<i32> = parts[0][2..]
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            let v: Vec<i32> = parts[1][2..]
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            ((p[0], p[1]), (v[0], v[1]))
        })
        .collect();

    let mut seconds = 0;

    loop {
        if seconds > 10_000 {
            println!("Simulation terminated: exceeded 10,000 seconds.");
            break;
        }
        for (pos, vel) in &mut robots {
            pos.0 = (pos.0 + vel.0).rem_euclid(width as i32);
            pos.1 = (pos.1 + vel.1).rem_euclid(height as i32);
        }        

        let mut grid = vec![vec!['.'; width]; height];
        for (pos, _) in &robots {
            grid[pos.1 as usize][pos.0 as usize] = '#';
        }

        if !grid.iter().any(|row| row.windows(10).any(|slice| slice.iter().all(|&c| c == '#'))) {
            seconds += 1;
            continue;
        }
        println!("\nTime: {} seconds", seconds+1);
        break;
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
