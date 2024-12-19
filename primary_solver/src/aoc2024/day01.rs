use std::collections::BinaryHeap;
use std::collections::HashMap;
use tokio::io;

pub async fn solve_a(
    lines: &Vec<String>,
)-> io::Result<()>{
    println!("Solving Day 1, Part A");
    let mut pq1 = BinaryHeap::new();
    let mut pq2 = BinaryHeap::new();
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let (Some(&num1_str), Some(&num2_str)) = (parts.get(0), parts.get(1)) {
            if let (Ok(num1), Ok(num2)) = (num1_str.parse::<i32>(), num2_str.parse::<i32>()) {
                pq1.push(num1);
                pq2.push(num2);
            } else {
                eprintln!("Error parsing numbers in line: {}", line);
            }
        } else {
            eprintln!("Error splitting line: {}", line);
        }
    }
    let total_diff = calculate_sum_of_abs_differences(&pq1, &pq2);

    println!("Total sum of absolute differences: {}", total_diff);
    Ok(())

}

fn calculate_sum_of_abs_differences(heap1: &BinaryHeap<i32>, heap2: &BinaryHeap<i32>) -> u64 {
    let sorted1: Vec<i32> = heap1.clone().into_sorted_vec();
    let sorted2: Vec<i32> = heap2.clone().into_sorted_vec();

    let mut sum = 0;
    for i in 0..sorted1.len() {
        sum += (sorted1[i] - sorted2[i]).abs() as u64;
    }

    sum
}

pub async fn solve_b(
    input_lines: &Vec<String>,
) -> io::Result<()>{
    println!("Solving Day 1, Part A");
    let lines = input_lines.to_vec();
   
    let mut first_numbers: Vec<i32> = Vec::new();
    let mut second_number_counts: HashMap<i32, u32> = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let (Some(&num1_str), Some(&num2_str)) = (parts.get(0), parts.get(1)) {
            if let (Ok(num1), Ok(num2)) = (num1_str.parse::<i32>(), num2_str.parse::<i32>()) {
                first_numbers.push(num1);

                *second_number_counts.entry(num2).or_insert(0) += 1;
            } else {
                eprintln!("Error parsing numbers in line: {}", line);
            }
        } else {
            eprintln!("Error splitting line: {}", line);
        }
    }
     let mut total: u64 = 0;

     for &item in &first_numbers {
         if let Some(&count) = second_number_counts.get(&item) {
             total += (item as u64) * (count as u64);
         }
     }
 
     println!("Total: {}", total);
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
