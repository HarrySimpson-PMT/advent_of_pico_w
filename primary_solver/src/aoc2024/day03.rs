use regex::Regex;
use tokio::io;

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 3, Part A");
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();

    let content = lines.join("\n");
    let matches: Vec<&str> = re.find_iter(&content).map(|mat| mat.as_str()).collect();

    let mut result = 0;
    for m in matches {
        let nums: Vec<&str> = m.split(",").collect();
        let a: i32 = nums[0].replace("mul(", "").parse().unwrap();
        let b: i32 = nums[1].replace(")", "").parse().unwrap();
        result += a * b;
    }

    println!("Result: {}", result);

    Ok(())
}

pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 3, Part B");
    let re = match Regex::new(r"mul\(\d+,\d+\)|don't\(\)|do\(\)") {
        Ok(re) => re,
        Err(err) => {
            println!("Error: {}", err);
            return Ok(());
        }
    };

    let content = lines.join("\n");
    let matches: Vec<&str> = re.find_iter(&content).map(|mat| mat.as_str()).collect();

    let mut result = 0;
    let mut on = true;
    for m in matches {
        if m == "do()" {
            on = true;
            continue;
        }
        if m == "don't()" {
            on = false;
            continue;
        }
        if !on {
            continue;
        }
        let nums: Vec<&str> = m.split(",").collect();
        let a: i32 = nums[0].replace("mul(", "").parse().unwrap();
        let b: i32 = nums[1].replace(")", "").parse().unwrap();
        result += a * b;
    }

    println!("Result: {}", result);

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
