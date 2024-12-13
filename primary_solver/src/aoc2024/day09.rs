use tokio::io;

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 9, Part A");

    // Assuming the input is the first line in the vector
    if let Some(input) = lines.get(0) {
        let mut output: Vec<i32> = Vec::new();
        let mut first = true;
        let mut cur = 0;

        // Build the initial vector with integers
        for ch in input.chars() {
            if let Some(digit) = ch.to_digit(10) {
                for _ in 0..digit {
                    if first {
                        output.push(cur); // Push the current number
                    } else {
                        output.push(-1); // Use -1 to represent '.'
                    }
                }
            }
            if first {
                cur += 1;
            }
            first = !first;
        }

        println!("Original Output: {:?}", output);

        // Swap logic: Swap the leftmost -1 with the rightmost non--1
        let mut left = 0;
        let mut right = output.len() - 1;

        while left < right {
            // Find the leftmost -1
            while left < output.len() && output[left] != -1 {
                left += 1;
            }
            // Find the rightmost non--1
            while right > 0 && output[right] == -1 {
                right -= 1;
            }
            // Swap if valid indices
            if left < right {
                output.swap(left, right);
                left += 1;
                right -= 1;
            }
        }

        println!("Swapped Output: {:?}", output);

        // Calculate the result
        let mut result: i64 = 0;
        for (i, &val) in output.iter().enumerate() {
            if val != -1 {
                result += val as i64 * i as i64;
            }
        }

        let result_str = result.to_string();
        // Print result number and the string
        println!("Result: {}", result);
        println!("Result String: {}", result_str);

    } else {
        println!("No input provided");
    }

    Ok(())
}

pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("--- Day 9: Disk Fragmenter ---");

    if let Some(input) = lines.get(0) {

        // Parse the disk map into a Vec<i32> where -1 represents free space
        let mut layout = parse_disk_map(input);


        // println!("Initial Layout: {}", display_layout(&layout));

        // Compact the disk using the whole file movement strategy
        compact_disk(&mut layout);

        // println!("Compacted Layout: {}", display_layout(&layout));

        // Calculate the checksum
        let checksum = calculate_checksum(&layout);
        println!("Checksum: {}", checksum);

        Ok(())
    } else {
        println!("No input provided");
        Ok(())
    }
}

fn parse_disk_map(input: &str) -> Vec<i32> {
    let mut layout = Vec::new();
    let mut file_id = 0;
    
    let digits = input.chars().map(|ch| ch.to_digit(10).unwrap());
    for (i, length) in digits.enumerate() {
        if i % 2 == 0 {
            // File blocks
            layout.extend(vec![file_id; length as usize]);
            file_id += 1;
        } else {
            // Free space blocks
            layout.extend(vec![-1; length as usize]);
        }
    }

    layout
}

fn compact_disk(layout: &mut Vec<i32>) {
    // Find unique file IDs in descending order
    let mut file_ids: Vec<i32> = layout.iter().filter(|&&x| x >= 0).copied().collect();
    file_ids.sort_unstable();
    file_ids.dedup();
    file_ids.reverse();

    for &file_id in &file_ids {
        // Determine file size and current position of the file
        let file_size = layout.iter().filter(|&&x| x == file_id).count();
        let current_start = layout.iter().position(|&x| x == file_id).unwrap();

        // Precompute spans of free space
        let mut free_spans = Vec::new();
        let mut start = None;

        for (i, &block) in layout.iter().enumerate() {
            if block == -1 {
                if start.is_none() {
                    start = Some(i);
                }
            } else if let Some(s) = start {
                free_spans.push((s, i - s)); // (start, length)
                start = None;
            }
        }
        if let Some(s) = start {
            free_spans.push((s, layout.len() - s));
        }

        // Find the leftmost span that fits and is to the left
        for &(start, length) in &free_spans {
            if length >= file_size && start < current_start {
                // Move the file
                for block in layout.iter_mut() {
                    if *block == file_id {
                        *block = -1;
                    }
                }
                for i in start..start + file_size {
                    layout[i] = file_id;
                }
                break;
            }
        }
    }
}


fn calculate_checksum(layout: &[i32]) -> i128 {
    layout
        .iter()
        .enumerate()
        .filter(|&(_, &file_id)| file_id >= 0)
        .map(|(pos, &file_id)| pos as i128 * file_id as i128) // Use i128
        .sum()
}

#[allow(dead_code)]
fn display_layout(layout: &[i32]) -> String {
    layout
        .iter()
        .map(|&x| if x == -1 { '.' } else { (x as u8 + b'0') as char })
        .collect()
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
