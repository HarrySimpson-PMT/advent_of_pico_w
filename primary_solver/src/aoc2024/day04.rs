use tokio::io;

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 4, Part A");
    let target = "XMAS";
    let found_count = count_occurrences(&lines, target);

    println!("Found {} occurrences of '{}'", found_count, target);

    Ok(())
}
fn count_occurrences(matrix: &Vec<String>, target: &str) -> usize {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let directions = [
        (0, 1),   // Right
        (1, 0),   // Down
        (0, -1),  // Left
        (-1, 0),  // Up
        (1, 1),   // Down-right
        (1, -1),  // Down-left
        (-1, 1),  // Up-right
        (-1, -1), // Up-left
    ];

    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            if matrix[row].as_bytes()[col] == b'X' {
                for &(dx, dy) in &directions {
                    if check_direction(matrix, target, row as isize, col as isize, dx, dy) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn check_direction(
    matrix: &Vec<String>,
    target: &str,
    start_row: isize,
    start_col: isize,
    dx: isize,
    dy: isize,
) -> bool {
    let rows = matrix.len() as isize;
    let cols = matrix[0].len() as isize;
    let target_bytes = target.as_bytes();

    for (i, &ch) in target_bytes.iter().enumerate() {
        let new_row = start_row + i as isize * dx;
        let new_col = start_col + i as isize * dy;

        if new_row < 0
            || new_row >= rows
            || new_col < 0
            || new_col >= cols
            || matrix[new_row as usize].as_bytes()[new_col as usize] != ch
        {
            return false;
        }
    }

    true
}

pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 4, Part B");
    let pattern_count = find_pattern(&lines);

    println!("Found {} patterns", pattern_count);

    Ok(())
}

fn find_pattern(matrix: &Vec<String>) -> usize {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut count = 0;

    let diagonals = [
        (-1, -1), // Top-left
        (-1, 1),  // Top-right
        (1, -1),  // Bottom-left
        (1, 1),   // Bottom-right
    ];

    for row in 0..rows {
        for col in 0..cols {
            if matrix[row].as_bytes()[col] == b'A' {
                let mut diag_chars = Vec::new();
                for &(dx, dy) in &diagonals {
                    let new_row = row as isize + dx;
                    let new_col = col as isize + dy;

                    if new_row >= 0
                        && new_row < rows as isize
                        && new_col >= 0
                        && new_col < cols as isize
                    {
                        diag_chars.push(matrix[new_row as usize].as_bytes()[new_col as usize]);
                    }
                }
                if diag_chars.len() == 4 {
                    let m_count = diag_chars.iter().filter(|&&c| c == b'M').count();
                    let s_count = diag_chars.iter().filter(|&&c| c == b'S').count();
                    if m_count == 2
                        && s_count == 2
                        && diag_chars[0] != diag_chars[3] // Top-left != Bottom-left
                    {
                        count += 1;
                    }
                }
            }
        }
    }

    count
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
