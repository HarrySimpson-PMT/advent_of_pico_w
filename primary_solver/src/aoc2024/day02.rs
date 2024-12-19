use tokio::io;

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 2, Part A");
    let mut goodlines: u32 = 0;
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mut is_valid: bool = true;
        let first: u32 = parts[0].parse().unwrap();
        let second: u32 = parts[1].parse().unwrap();
        let ascending: bool = first < second;
        for i in 1..parts.len() {
            let prev: u32 = parts[i - 1].parse().unwrap();
            let curr: u32 = parts[i].parse().unwrap();
            if ascending {
                if curr <= prev || curr-prev  > 3 {
                    is_valid = false;
                }
            } else {
                if curr >= prev || prev-curr  > 3 {
                    is_valid = false;
                }
            }
        }
        if is_valid {
            goodlines += 1;
            // print!("Good line: {}", line);
        }
        else {
            // print!("Bad line: {}", line);
        }
        println!();
    }
    print!("Good lines: {}", goodlines);

    Ok(())
}

pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 2, Part A");
    let mut goodlines: u32 = 0;
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mut is_valid: bool = true;
        let first: u32 = parts[0].parse().unwrap();
        let second: u32 = parts[1].parse().unwrap();
        let ascending: bool = first < second;
        for i in 1..parts.len() {
            let prev: u32 = parts[i - 1].parse().unwrap();
            let curr: u32 = parts[i].parse().unwrap();
            if ascending {
                if curr <= prev || curr-prev  > 3 {
                    is_valid = false;
                }
            } else {
                if curr >= prev || prev-curr  > 3 {
                    is_valid = false;
                }
            }
            // print!("{} {} {} {} ", prev, curr, is_valid, ascending);
        }
        if is_valid {
            goodlines += 1;
            // print!("Good line: {}", line);
        }
        else {
            for i in 0..parts.len() {
                let mut new_parts = parts.clone();
                new_parts.remove(i);
                let mut is_valid: bool = true;
                let first: u32 = new_parts[0].parse().unwrap();
                let second: u32 = new_parts[1].parse().unwrap();
                let ascending: bool = first < second;
                for i in 1..new_parts.len() {
                    let prev: u32 = new_parts[i - 1].parse().unwrap();
                    let curr: u32 = new_parts[i].parse().unwrap();
                    if ascending {
                        if curr <= prev || curr-prev  > 3 {
                            is_valid = false;
                        }
                    } else {
                        if curr >= prev || prev-curr  > 3 {
                            is_valid = false;
                        }
                    }
                    // print!("{} {} {} {} ", prev, curr, is_valid, ascending);
                }
                if is_valid {
                    goodlines += 1;
                    break;
                    // print!("Good line: {}", line);
                }
                else {
                    // print!("Bad line: {}", line);
                }
            }
        }
    }
    print!("Good lines: {}", goodlines);

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
