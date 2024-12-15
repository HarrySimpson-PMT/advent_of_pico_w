use tokio::io;

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 15, Part A");

    let mut parts = lines.split(|line| line.trim().is_empty());
    let map_input = parts.next().unwrap();
    let moves_input = parts.next().unwrap();

    let mut transformed_grid: Vec<Vec<char>> = map_input
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let moves: String = moves_input.join("").replace('\n', "");

    let height = transformed_grid.len();
    let width = transformed_grid[0].len();

    let mut robot_pos = (0, 0);
    for (row, line) in transformed_grid.iter().enumerate() {
        if let Some(col) = line.iter().position(|&c| c == '@') {
            robot_pos = (row, col);
            break;
        }
    }

    let directions = vec![('<', (0, -1)), ('>', (0, 1)), ('^', (-1, 0)), ('v', (1, 0))];

    for mov in moves.chars() {
        let (dr, dc) = directions.iter().find(|&&(d, _)| d == mov).unwrap().1;
        let (new_r, new_c) = (
            robot_pos.0 as isize + dr as isize,
            robot_pos.1 as isize + dc as isize,
        );

        if new_r < 0 || new_c < 0 || new_r >= height as isize || new_c >= width as isize {
            continue;
        }

        let new_r = new_r as usize;
        let new_c = new_c as usize;

        match transformed_grid[new_r][new_c] {
            '.' => {
                transformed_grid[robot_pos.0][robot_pos.1] = '.';
                robot_pos = (new_r, new_c);
                transformed_grid[robot_pos.0][robot_pos.1] = '@';
            }
            'O' => {
                let mut r = new_r as isize;
                let mut c = new_c as isize;
                let mut found = false;
                loop {
                    r += dr as isize;
                    c += dc as isize;
                    if r < 0 || c < 0 || r >= height as isize || c >= width as isize {
                        break;
                    }
                    let ur = r as usize;
                    let uc = c as usize;
                    match transformed_grid[ur][uc] {
                        '.' => {
                            found = true;
                            break;
                        }
                        '#' => {
                            break;
                        }
                        _ => {}
                    }
                }
                if found {
                    transformed_grid[robot_pos.0][robot_pos.1] = '.';
                    robot_pos = (new_r, new_c);
                    transformed_grid[robot_pos.0][robot_pos.1] = '@';
                    let ur = r as usize;
                    let uc = c as usize;
                    transformed_grid[ur][uc] = 'O';
                }
            }
            '#' => {
                continue;
            }
            _ => {}
        }
    }

    let mut gps_sum = 0;
    for (row, line) in transformed_grid.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            if cell == 'O' {
                gps_sum += 100 * row + col;
            }
        }
    }

    println!("Sum of GPS coordinates: {}", gps_sum);

    Ok(())
}

pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 15, Part B");

    let mut parts = lines.split(|line| line.trim().is_empty());
    let map_input = parts.next().unwrap();
    let moves_input = parts.next().unwrap();

    let grid: Vec<Vec<char>> = map_input
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut transformed_grid = transform_map(grid);

    let height = transformed_grid.len();
    let width = transformed_grid[0].len();

    let mut robot_pos = (0, 0);
    for (row, line) in transformed_grid.iter().enumerate() {
        if let Some(col) = line.iter().position(|&c| c == '@') {
            robot_pos = (row, col);
            break;
        }
    }

    let moves: String = moves_input.join("").replace('\n', "");
    for row in &transformed_grid {
        println!("{}", row.iter().collect::<String>());
    }
    let directions = vec![('<', (0, -1)), ('>', (0, 1)), ('^', (-1, 0)), ('v', (1, 0))];

    for mov in moves.chars() {
        let (dr, dc) = directions.iter().find(|&&(d, _)| d == mov).unwrap().1;
        let (new_r, new_c) = (
            robot_pos.0 as isize + dr as isize,
            robot_pos.1 as isize + dc as isize,
        );

        if new_r < 0 || new_c < 0 || new_r >= height as isize || new_c >= width as isize {
            continue;
        }

        let new_r = new_r as usize;
        let new_c = new_c as usize;

        match transformed_grid[new_r][new_c] {
            '.' => {
                transformed_grid[robot_pos.0][robot_pos.1] = '.';
                robot_pos = (new_r, new_c);
                transformed_grid[robot_pos.0][robot_pos.1] = '@';
            }
            '[' | ']' => match mov {
                '<' | '>' => {
                    let mut r = new_r as isize;
                    let mut c = new_c as isize;
                    let mut found = false;

                    loop {
                        r += dr as isize;
                        c += dc as isize;
                        if c < 0 || c >= width as isize {
                            break;
                        }

                        let uc = c as usize;
                        match transformed_grid[r as usize][uc] {
                            '.' => {
                                found = true;
                                break;
                            }
                            '[' | ']' => continue,
                            '#' => break,
                            _ => {}
                        }
                    }

                    if found {
                        println!("Found a path");
                        while r as usize != new_r || c as usize != new_c {
                            let ur = r as usize;
                            let uc = c as usize;
                            let (check_r, check_c) = ((r - dr) as usize, (c - dc) as usize);
                            transformed_grid[ur][uc] = transformed_grid[check_r][check_c];
                            r -= dr;
                            c -= dc;
                        }

                        transformed_grid[robot_pos.0][robot_pos.1] = '.';
                        robot_pos = (new_r, new_c);
                        transformed_grid[robot_pos.0][robot_pos.1] = '@';
                    }
                }
                '^' | 'v' => {
                    let mut queue = vec![];
                    let mut found = true;
                    let mut box_parts = std::collections::HashSet::new();

                    queue.push((robot_pos.0 as isize, robot_pos.1 as isize));

                    println!("Queue: {:?}", queue);

                    while let Some((r, c)) = queue.pop() {
                        let next_r = r + dr as isize;
                        let next_c = c + dc as isize;
                    
                        if next_r < 0 || next_c < 0 || next_r >= height as isize || next_c >= width as isize {
                            continue;
                        }
                    
                        let ur = next_r as usize;
                        let uc = next_c as usize;
                    
                        match transformed_grid[ur][uc] {
                            '[' => {
                                if box_parts.insert(((ur, uc), '[')) {
                                    queue.push((r + dr as isize, c + dc as isize));
                                }
                                if box_parts.insert(((ur, uc + 1), ']')) {
                                    queue.push((r + dr as isize, c + dc as isize + 1));
                                }
                            }
                            ']' => {
                                if box_parts.insert(((ur, uc), ']')) {
                                    queue.push((r + dr as isize, c + dc as isize));
                                }
                                if box_parts.insert(((ur, uc - 1), '[')) {
                                    queue.push((r + dr as isize, c + dc as isize - 1));
                                }
                            }
                            '#' => {
                                println!("Found a wall at: {}, {}", ur, uc);
                                found = false;
                                break;
                            }
                            '.' => {
                            }
                            _ => {}
                        }
                    }
                    println!("Found: {}", found);

                    if found {
                        println!("box_parts: {:?}", box_parts);
                        for &((ur, uc), _) in &box_parts {
                            transformed_grid[ur][uc] = '.';
                        }

                        for row in &transformed_grid {
                            println!("{}", row.iter().collect::<String>());
                        }

                        let updated_parts: Vec<((usize, usize), char)> = box_parts
                            .into_iter()
                            .map(|((ur, uc), val)| {
                                (
                                    ((ur as isize + dr) as usize, (uc as isize + dc) as usize),
                                    val,
                                )
                            })
                            .collect();

                        for &((ur, uc), val) in &updated_parts {
                            transformed_grid[ur][uc] = val;
                        }

                        transformed_grid[robot_pos.0][robot_pos.1] = '.';
                        robot_pos = (new_r, new_c);
                        transformed_grid[robot_pos.0][robot_pos.1] = '@';
                    }
                }
                _ => {}
            },

            '#' => {
                continue;
            }
            _ => {}
        }
    }

    //print map
    for row in &transformed_grid {
        println!("{}", row.iter().collect::<String>());
    }

    let mut gps_sum = 0;

    for (row, line) in transformed_grid.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            if cell == '[' {
                gps_sum += 100 * row + col; // Use the top-left corner of the box for GPS
            }
        }
    }
    
    println!("Sum of GPS coordinates: {}", gps_sum);
    Ok(())
}

fn transform_map(transformed_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_transformed_grid = Vec::new();

    for row in transformed_grid {
        let mut new_row = Vec::new();
        for cell in row {
            match cell {
                '#' => new_row.extend(vec!['#', '#']),
                'O' => new_row.extend(vec!['[', ']']),
                '.' => new_row.extend(vec!['.', '.']),
                '@' => new_row.extend(vec!['@', '.']),
                _ => {}
            }
        }
        new_transformed_grid.push(new_row);
    }

    new_transformed_grid
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
