use std::collections::{HashMap, HashSet};
use tokio::io;

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 5, Part A");

    // Split the vector into two parts based on an empty line
    let mut iter = lines.split(|line| line.is_empty());
    let first_part: Vec<String> = iter.next().unwrap().to_vec();
    let second_part: Vec<String> = iter.next().unwrap().to_vec();

    let dictionary = populate_dictionary(&first_part);
    let result = calculate_results_from_input(&second_part, &dictionary);

    println!("Result: {}", result);

    Ok(())
}

pub fn populate_dictionary(input: &[String]) -> HashMap<i32, Vec<i32>> {
    let mut dictionary: HashMap<i32,  Vec<i32>> = HashMap::new();

    for line in input {
        if let Some((x_str, y_str)) = line.split_once('|') {
            // Parse the key and value
            if let (Ok(x), Ok(y)) = (x_str.parse::<i32>(), y_str.parse::<i32>()) {
                // Add `y` to the `pro` vector of key `x`
                dictionary.entry(y).or_default().push(x);

            }
        }
    }
    dictionary
}

pub fn calculate_results_from_input(
    second_part: &[String],
    dictionary: &HashMap<i32, Vec<i32>>,
) -> i32 {
    let (valid_lines, _) = split_lines(second_part, dictionary);

    valid_lines
        .iter()
        .map(|line| calculate_middle(line))
        .sum()
}
pub fn split_lines(
    second_part: &[String],
    dictionary: &HashMap<i32, Vec<i32>>,
) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut valid_lines = Vec::new();
    let mut invalid_lines = Vec::new();

    for line in second_part {
        let numbers: Vec<i32> = line
            .split(',')
            .filter_map(|num| num.trim().parse::<i32>().ok())
            .collect();

        if check_validity(&numbers, dictionary) {
            valid_lines.push(numbers);
        } else {
            invalid_lines.push(numbers);
        }
    }

    (valid_lines, invalid_lines)
}

pub fn check_validity(
    numbers: &[i32],
    dictionary: &HashMap<i32, Vec<i32>>,
) -> bool {
    let mut visited: HashSet<i32> = HashSet::new();
    let mut not_visited: HashSet<i32> = numbers.iter().cloned().collect();

    for &num in numbers {
        if let Some(entry) = dictionary.get(&num) {
            // Ensure none of the `pre` entries are in `not_visited`
            let pre_ok = entry.iter().all(|&pre_key| !not_visited.contains(&pre_key));
            if !pre_ok {
                return false; // Invalid if any pre condition fails
            }
        }
        // Mark current number as visited
        visited.insert(num);
        not_visited.remove(&num);
    }
    true
}
pub fn calculate_middle(numbers: &[i32]) -> i32 {
    if numbers.is_empty() {
        0
    } else {
        let middle_index = numbers.len() / 2;
        numbers[middle_index]
    }
}


pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 5, Part B");

    // Split the vector into two parts based on an empty line
    let mut iter = lines.split(|line| line.is_empty());
    let first_part: Vec<String> = iter.next().unwrap().to_vec();
    let second_part: Vec<String> = iter.next().unwrap().to_vec();

    let dictionary = populate_dictionary(&first_part);

    // Split the lines into valid and invalid
    let (_, invalid_lines) = split_lines(&second_part, &dictionary);

    // Fix the invalid lines
    let result = fix_invalid_lines(&invalid_lines, &dictionary);

    println!("Result: {}", result);

    Ok(())
}



pub fn topological_sort_line(numbers: &[i32], dictionary: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut in_degree: HashMap<i32, usize> = HashMap::new();
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();

    // Build the subgraph from the dictionary using the numbers in the line
    for &num in numbers {
        if let Some(dependencies) = dictionary.get(&num) {
            graph.entry(num).or_default().extend(dependencies.iter().cloned());

            // Initialize in-degree for `num`
            in_degree.entry(num).or_insert(0);

            // Increment in-degree for dependencies
            for &dep in dependencies {
                if numbers.contains(&dep) {
                    *in_degree.entry(dep).or_insert(0) += 1;
                }
            }
        }
    }

    // Find all nodes with in-degree 0
    let mut queue: Vec<i32> = in_degree
        .iter()
        .filter(|&(_, &degree)| degree == 0)
        .map(|(&node, _)| node)
        .collect();

    let mut sorted_order = Vec::new();

    // Perform topological sort
    while let Some(node) = queue.pop() {
        sorted_order.push(node);

        if let Some(edges) = graph.remove(&node) {
            for edge in edges {
                if let Some(degree) = in_degree.get_mut(&edge) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push(edge);
                    }
                }
            }
        }
    }

    sorted_order
}

pub fn fix_invalid_lines(
    invalid_lines: &[Vec<i32>],
    dictionary: &HashMap<i32, Vec<i32>>,
) -> i32 {
    let mut results = 0;

    for numbers in invalid_lines {
        // Topologically sort the invalid line
        let corrected_line = topological_sort_line(numbers, dictionary);

        // Calculate the middle value of the corrected line
        if !corrected_line.is_empty() {
            let middle_index = corrected_line.len() / 2;
            results += corrected_line[middle_index];
        }
    }

    results
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
