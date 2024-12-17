use tokio::io;

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 17, Part A");

    // Parse input
    let program: Vec<u8> = lines
        .iter()
        .filter(|line| line.starts_with("Program:"))
        .flat_map(|line| line["Program: ".len()..].split(','))
        .map(|x| x.parse::<u8>().unwrap())
        .collect();
    let register_a = lines[0]
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .parse::<i64>()
        .unwrap();

    let output = execute_program(register_a, &program);
    println!("Output: {:?}", output);

    Ok(())
}

fn execute_program(initial_a: i64, program: &Vec<u8>) -> Vec<u8> {
    let mut registers = [initial_a, 0i64, 0i64];
    let mut ip = 0usize;
    let mut output = Vec::new();

    while ip < program.len() {
        let opcode = program[ip];
        let operand = program[ip + 1];

        match opcode {
            0 => registers[0] /= 2_i64.pow(get_value(operand, &registers) as u32),
            1 => registers[1] ^= operand as i64,
            2 => registers[1] = get_value(operand, &registers) % 8,
            3 => {
                if registers[0] != 0 {
                    ip = operand as usize;
                    continue;
                }
            }
            4 => registers[1] ^= registers[2],
            5 => output.push((get_value(operand, &registers) % 8) as u8),
            6 => registers[1] = registers[0] / 2_i64.pow(get_value(operand, &registers) as u32),
            7 => registers[2] = registers[0] / 2_i64.pow(get_value(operand, &registers) as u32),
            _ => break,
        }

        ip += 2;
    }

    output
}

fn get_value(operand: u8, registers: &[i64; 3]) -> i64 {
    match operand {
        0..=3 => operand as i64,
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        _ => panic!("Invalid operand"),
    }
}

pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 17, Part B");

    let program: Vec<u8> = lines
        .iter()
        .filter(|line| line.starts_with("Program:"))
        .flat_map(|line| line["Program: ".len()..].split(','))
        .map(|x| x.parse::<u8>().unwrap())
        .collect();

    let test = binary_search_length(&program);
    // println!("Test: {:?}", test);
    // let mut low: i64 = 135184372088838;
    // let mut hight: i64 = 185184372088838;
    let mut low: i64 = test.0;
    let mut hight: i64 = test.1;
    
    let step = scan_and_reduce(low, hight, program.len(), &program, 1000);
    // println!("Step: {:?}", step);

    let mut result = 0;
    for i in (0..program.len()).rev() {
        let step = scan_and_reduce(low, hight, i, &program, 200);
        // println!("Step: {:?}", step);
        if step.0 == step.1{
            result = step.0;
            break;
        }
        low = step.0;
        hight = step.1;
    }

    println!("Result: {:?}", result);

    Ok(())
}

fn binary_search_length(program: &Vec<u8>) -> (i64, i64) {
    let mut low: i64 = 0;
    let mut high: i64 = i64::MAX;
    let mut right:i64 = i64::MAX;

    while low < high {
        let mid = low + (high - low) / 2;
        let output = execute_program(mid, program);
        // println!("mid: {:?} -> {:?}", mid, output);
        if output.len() == program.len()+1{

            right = mid;
        }
        if output.len() == program.len()-1{
            return (mid, right);
        }
        if output.len() < program.len() {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    panic!("No match found for position {}", program.len());
}

fn scan_and_reduce(
    low: i64,
    high: i64,
    position: usize,
    program: &Vec<u8>,
    scan_points: usize,
) -> (i64, i64) {
    let step = (high - low) / scan_points as i64;

    for i in 0..=scan_points {
        let test_a = low + (i as i64 * step).min(high - low);
        let output = execute_program(test_a, program);

        if matches_from_position(&output, program, position) {

            let mut refined_low = test_a;
            while refined_low < high {
                let refined_step = 1.max((high - refined_low) / 10); 
                let next_a = refined_low + refined_step;
                let output = execute_program(next_a, program);

                if !matches_from_position(&output, program, position) {
                    refined_low = next_a;
                    break; 
                }
                refined_low = next_a;
            }            

            return (test_a - step, refined_low + step);
        }
    }
    for i in low..high {
        let output = execute_program(i, program);
        if matches_from_position(&output, program, 1) {
            return (i, i );
        }
    }    
    panic!("No match found for position {}", position);
}

fn matches_from_position(output: &Vec<u8>, program: &Vec<u8>, position: usize) -> bool {
    if output.len() < position || output.len() > program.len() {
        return false; // 
    }
    for i in position-1..program.len() {
        if output.get(i) != Some(&program[i]) {
            return false; 
        }
    }

    true
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
