use tokio::io;

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 13, Part A");

    let machines = parse_input(lines);

    let total_cost: i64 = machines
        .iter()
        .map(|machine| {
            let (n, m) = solve_machine(machine, 0);
            n * 3 + m * 1
        })
        .sum();

    println!("Total cost for Part B: {}", total_cost);

    Ok(())
}

#[derive(Debug)]
struct Machine {
    button_a_x: i64,
    button_a_y: i64,
    button_b_x: i64,
    button_b_y: i64,
    target_x: i64,
    target_y: i64,
}

fn parse_input(lines: &Vec<String>) -> Vec<Machine> {
    lines
        .chunks(4)
        .map(|chunk| {
            let button_a = parse_button(&chunk[0]);
            let button_b = parse_button(&chunk[1]);
            let prize = parse_prize(&chunk[2]);
            Machine {
                button_a_x: button_a.0,
                button_a_y: button_a.1,
                button_b_x: button_b.0,
                button_b_y: button_b.1,
                target_x: prize.0,
                target_y: prize.1,
            }
        })
        .collect()
}

fn parse_button(line: &str) -> (i64, i64) {
    let parts: Vec<_> = line.split_whitespace().collect();
    let x = parts[2].replace("X+", "").replace(",", "").parse().unwrap();
    let y = parts[3].replace("Y+", "").parse().unwrap();
    (x, y)
}

fn parse_prize(line: &str) -> (i64, i64) {
    let parts: Vec<_> = line.split_whitespace().collect();
    let x = parts[1].replace("X=", "").replace(",", "").parse().unwrap();
    let y = parts[2].replace("Y=", "").parse().unwrap();
    (x, y)
}


pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 13, Part B");

    let machines = parse_input(lines);

    let total_cost: i64 = machines
        .iter()
        .map(|machine| {
            let (n, m) = solve_machine(machine, 10000000000000);
            n * 3 + m * 1
        })
        .sum();

    println!("Total cost for Part B: {}", total_cost);

    Ok(())
}
/// Diophantine equation
fn solve_machine(machine: &Machine, factor: i64) -> (i64, i64) {
    let determinate = (machine.button_a_x * machine.button_b_y) - (machine.button_a_y * machine.button_b_x);
    if determinate == 0 {
        panic!("det == 0");
    }
    let button_a: i64 = (machine.button_b_y * (machine.target_x + factor) 
        - machine.button_b_x * (machine.target_y + factor)) 
        / determinate;
    let button_b = (machine.button_a_x * (machine.target_y + factor) 
        - machine.button_a_y * (machine.target_x + factor)) 
        / determinate;

    let a_correct = button_a * machine.button_a_x + button_b * machine.button_b_x == machine.target_x + factor;
    let b_correct = button_a * machine.button_a_y + button_b * machine.button_b_y == machine.target_y + factor;
    if a_correct && b_correct {
        (button_a, button_b)
    } else {
        (0, 0)
    }
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
