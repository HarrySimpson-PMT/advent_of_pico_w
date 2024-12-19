use tokio::io;
use std::collections::{HashSet, HashMap};

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    let mut sections = lines.split(|line| line.is_empty());
    let towel_patterns: HashSet<String> = sections
        .next()
        .unwrap_or_default()
        .first()
        .unwrap_or(&String::new())
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    let designs: Vec<&String> = sections.next().unwrap_or_default().iter().collect();

    fn can_construct(design: &str, patterns: &HashSet<String>) -> bool {
        let mut dp = vec![false; design.len() + 1];
        dp[0] = true;
        for i in 0..design.len() {
            if dp[i] {
                for pattern in patterns {
                    if i + pattern.len() <= design.len() && &design[i..i + pattern.len()] == pattern {
                        dp[i + pattern.len()] = true;
                    }
                }
            }
        }
        dp[design.len()]
    }

    let possible_count = designs.iter().filter(|&design| can_construct(design, &towel_patterns)).count();
    println!("Number of possible designs: {}", possible_count);
    Ok(())
}

pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    let mut sections = lines.split(|line| line.is_empty());
    let towel_patterns: HashSet<String> = sections
        .next()
        .unwrap_or_default()
        .first()
        .unwrap_or(&String::new())
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    let designs: Vec<&String> = sections.next().unwrap_or_default().iter().collect();

    fn count_ways(design: &str, patterns: &HashSet<String>, memo: &mut HashMap<String, usize>) -> usize {
        if design.is_empty() {
            return 1;
        }
        if let Some(&cached) = memo.get(design) {
            return cached;
        }
        let total_ways = patterns.iter().filter(|&pattern| design.starts_with(pattern)).map(|pattern| count_ways(&design[pattern.len()..], patterns, memo)).sum();
        memo.insert(design.to_string(), total_ways);
        total_ways
    }

    let total_count: usize = designs.iter().map(|design| count_ways(design, &towel_patterns, &mut HashMap::new())).sum();
    println!("Total number of ways: {}", total_count);
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
