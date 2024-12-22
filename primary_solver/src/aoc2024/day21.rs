use std::collections::HashMap;
use tokio::io;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    row: i32,
    col: i32,
}
impl Position {
    fn new(row: i32, col: i32) -> Self {
        Position { row, col }
    }
}

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 21, Part A");

    let (numeric_keypad_map, directional_keypad_map) = build_maps();

    let north_west_a = "^<A".to_string();
    let north_west_b = "<^A".to_string();
    let north_west_a_gen1 = keypad_move(&north_west_a, &directional_keypad_map, true);
    let north_west_b_gen1 = keypad_move(&north_west_b, &directional_keypad_map, true);
    let north_west_a_gen2 = keypad_move(&north_west_a_gen1, &directional_keypad_map, true);
    let north_west_b_gen2 = keypad_move(&north_west_b_gen1, &directional_keypad_map, true);
    println!("{}: {}", north_west_a_gen2.len(), north_west_b_gen2.len());

    let north_east_a = "^>A".to_string();
    let north_east_b = ">^A".to_string();
    let north_east_a_gen1 = keypad_move(&north_east_a, &directional_keypad_map, true);
    let north_east_b_gen1 = keypad_move(&north_east_b, &directional_keypad_map, true);
    let north_east_a_gen2 = keypad_move(&north_east_a_gen1, &directional_keypad_map, true);
    let north_east_b_gen2 = keypad_move(&north_east_b_gen1, &directional_keypad_map, true);
    println!("{}: {}", north_east_a_gen2.len(), north_east_b_gen2.len());

    let south_west_a = "v<A".to_string();
    let south_west_b = "<vA".to_string();
    let south_west_a_gen1 = keypad_move(&south_west_a, &directional_keypad_map, true);
    let south_west_b_gen1 = keypad_move(&south_west_b, &directional_keypad_map, true);
    let south_west_a_gen2 = keypad_move(&south_west_a_gen1, &directional_keypad_map, true);
    let south_west_b_gen2 = keypad_move(&south_west_b_gen1, &directional_keypad_map, true);
    println!("{}: {}", south_west_a_gen2.len(), south_west_b_gen2.len());

    let south_east_a = "v>A".to_string();
    let south_east_b = ">vA".to_string();
    let south_east_a_gen1 = keypad_move(&south_east_a, &directional_keypad_map, true);
    let south_east_b_gen1 = keypad_move(&south_east_b, &directional_keypad_map, true);
    let south_east_a_gen2 = keypad_move(&south_east_a_gen1, &directional_keypad_map, true);
    let south_east_b_gen2 = keypad_move(&south_east_b_gen1, &directional_keypad_map, true);
    println!("{}: {}", south_east_a_gen2.len(), south_east_b_gen2.len());

    let mut total_sum = 0;

    for line in lines {
        let first = keypad_move(&line, &numeric_keypad_map, false);
        let second = keypad_move(&first, &directional_keypad_map, true);
        let third = keypad_move(&second, &directional_keypad_map, true);
        let len = third.len();
        let first_three = line.chars().take(3).collect::<String>();
        let numb = first_three.parse::<i64>().unwrap_or(0);
        let result: i64 = len as i64 * numb;
        println!("{}: {}", line, third);
        println!("{} * {} = {}", numb, len, result);

        total_sum += result;
    }

    let gentest = keypad_move(&"37".to_string(), &numeric_keypad_map, false);
    println!("{}: {}", gentest.len(), gentest);

    println!("Total Sum: {}", total_sum);
    Ok(())
}

// "<^"
// "<^"
// "<v"
// "v>"

fn keypad_move(input: &String, map: &HashMap<(char, char), String>, directional: bool) -> String {
    let test = format!("A{}", input);
    let mut testresult = String::new();
    for (start_key, end_key) in test.chars().zip(test.chars().skip(1)) {
        if let Some(movement) = map.get(&(start_key, end_key)) {
            testresult.push_str(&movement);
            testresult.push('A');
        } else if directional {
            testresult.push('A');
        }
    }
    testresult
}
fn keypad_pair(input: &String, map: &HashMap<(char, char), String>) -> String {
    if input.len() < 2 {
        return input.clone();
    }
    let mut testresult = String::new();
    for (start_key, end_key) in input.chars().zip(input.chars().skip(1)) {
        if start_key == end_key {
            testresult.push('A');
        } else if let Some(movement) = map.get(&(start_key, end_key)) {
            testresult.push_str(movement);
            testresult.push('A');
        } else {
            testresult.push('A');
        }
    }
    testresult
}

fn sort_movement(movement: &str, invert: bool) -> String {
    let mut chars: Vec<char> = movement.chars().collect();

    chars.sort_by_key(|&c| match c {
        '<' => {
            if invert {
                3
            } else {
                0
            }
        }
        'v' => {
            if invert {
                1
            } else {
                2
            }
        }
        '>' => {
            if invert {
                0
            } else {
                3
            }
        }
        '^' => {
            if invert {
                2
            } else {
                1
            }
        }
        _ => 4,
    });

    chars.iter().collect()
}

fn calculate_movement(start: Position, end: Position, is_directional: bool) -> String {
    let mut movement = String::new();

    let vertical_moves = end.row - start.row;
    let horizontal_moves = end.col - start.col;

    let mut test_pos = start;
    if horizontal_moves < 0 { // Moving left
        test_pos.col += horizontal_moves;
        if is_directional {
            if test_pos == Position::new(0, 0) {
                if vertical_moves < 0 {
                    movement.push_str(&"^".repeat(-vertical_moves as usize));
                } else {
                    movement.push_str(&"v".repeat(vertical_moves as usize));
                }
                movement.push_str(&"<".repeat(-horizontal_moves as usize));
                
            } else {
                movement.push_str(&"<".repeat(-horizontal_moves as usize));
                if vertical_moves < 0 {
                    movement.push_str(&"^".repeat(-vertical_moves as usize));
                } else {
                    movement.push_str(&"v".repeat(vertical_moves as usize));
                }
            }
        } else {
            println!("vertical moves: {}", vertical_moves);
            if test_pos == Position::new(3, 0) {
                if vertical_moves < 0 {
                    movement.push_str(&"^".repeat(-vertical_moves as usize));
                } else {
                    movement.push_str(&"v".repeat(vertical_moves as usize));
                }
                movement.push_str(&"<".repeat(-horizontal_moves as usize));
            } else {
                movement.push_str(&"<".repeat(-horizontal_moves as usize));
                if vertical_moves < 0 {
                    movement.push_str(&"^".repeat(-vertical_moves as usize));
                } else {
                    movement.push_str(&"v".repeat(vertical_moves as usize));
                }
            }
        }
    } else if vertical_moves < 0 { 
        test_pos.row += vertical_moves;
        if is_directional {
            if test_pos == Position::new(0, 0) {
                if horizontal_moves < 0 {
                    movement.push_str(&"<".repeat(-horizontal_moves as usize));
                } else {
                    movement.push_str(&">".repeat(horizontal_moves as usize));
                }
                movement.push_str(&"^".repeat(-vertical_moves as usize));
            } else {
                movement.push_str(&"^".repeat(-vertical_moves as usize));
                if horizontal_moves < 0 {
                    movement.push_str(&"<".repeat(-horizontal_moves as usize));
                } else {
                    movement.push_str(&">".repeat(horizontal_moves as usize));
                }
            }
        } else {
            movement.push_str(&"^".repeat(-vertical_moves as usize));
            if horizontal_moves < 0 {
                movement.push_str(&"<".repeat(-horizontal_moves as usize));
            } else {
                movement.push_str(&">".repeat(horizontal_moves as usize));
            }            
        }
    } else if vertical_moves > 0 { 
        test_pos.row += vertical_moves;
        if is_directional {
            movement.push_str(&"v".repeat(vertical_moves as usize));
            if horizontal_moves < 0 {
                movement.push_str(&"<".repeat(-horizontal_moves as usize));
            } else {
                movement.push_str(&">".repeat(horizontal_moves as usize));
            }
        } else {
            if test_pos == Position::new(3, 0) {
                if horizontal_moves < 0 {
                    movement.push_str(&"<".repeat(-horizontal_moves as usize));
                } else {
                    movement.push_str(&">".repeat(horizontal_moves as usize));
                }
                movement.push_str(&"v".repeat(vertical_moves as usize));
            } else {
                movement.push_str(&"v".repeat(vertical_moves as usize));
                if horizontal_moves < 0 {
                    movement.push_str(&"<".repeat(-horizontal_moves as usize));
                } else {
                    movement.push_str(&">".repeat(horizontal_moves as usize));
                }
            } 
        }
    } else if horizontal_moves > 0 { 
        movement.push_str(&">".repeat(horizontal_moves as usize));
    }
    movement
}

pub fn build_maps() -> (HashMap<(char, char), String>, HashMap<(char, char), String>) {
    // Define the numeric keypad layout
    let numeric_keypad = vec![
        vec![Some('7'), Some('8'), Some('9')],
        vec![Some('4'), Some('5'), Some('6')],
        vec![Some('1'), Some('2'), Some('3')],
        vec![None, Some('0'), Some('A')],
    ];

    // Define the directional keypad layout
    let directional_keypad = vec![
        vec![None, Some('^'), Some('A')],
        vec![Some('<'), Some('v'), Some('>')],
    ];

    // Create a mapping for every pair of key literals on the numeric keypad
    let mut numeric_keypad_map: HashMap<(char, char), String> = HashMap::new();
    for (start_row, row_vec) in numeric_keypad.iter().enumerate() {
        for (start_col, &start_key) in row_vec.iter().enumerate() {
            if let Some(start_key_literal) = start_key {
                for (end_row, end_row_vec) in numeric_keypad.iter().enumerate() {
                    for (end_col, &end_key) in end_row_vec.iter().enumerate() {
                        if let Some(end_key_literal) = end_key {
                            if start_key_literal != end_key_literal {
                                let start_position =
                                    Position::new(start_row as i32, start_col as i32);
                                let end_position = Position::new(end_row as i32, end_col as i32);
                                let movement =
                                    calculate_movement(start_position, end_position, false);
                                numeric_keypad_map
                                    .insert((start_key_literal, end_key_literal), movement);
                            }
                        }
                    }
                }
            }
        }
    }

    // Create a mapping for every pair of key literals on the directional keypad
    let mut directional_keypad_map = HashMap::new();
    for (start_row, row_vec) in directional_keypad.iter().enumerate() {
        for (start_col, &start_key) in row_vec.iter().enumerate() {
            if let Some(start_key_literal) = start_key {
                for (end_row, end_row_vec) in directional_keypad.iter().enumerate() {
                    for (end_col, &end_key) in end_row_vec.iter().enumerate() {
                        if let Some(end_key_literal) = end_key {
                            if start_key_literal != end_key_literal {
                                let start_position =
                                    Position::new(start_row as i32, start_col as i32);
                                let end_position = Position::new(end_row as i32, end_col as i32);
                                let movement =
                                    calculate_movement(start_position, end_position, true);
                                directional_keypad_map
                                    .insert((start_key_literal, end_key_literal), movement);
                            }
                        }
                    }
                }
            }
        }
    }

    (numeric_keypad_map, directional_keypad_map)
}

pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 21, Part B");
 
    let (numeric_keypad_map, directional_keypad_map) = build_maps();

    let mut genmap: HashMap<String, i64> = HashMap::new();
    let mut genmap2 = HashMap::new();
    for (start_key, end_key) in directional_keypad_map.keys() {
        let mut movement = directional_keypad_map.get(&(*start_key, *end_key)).unwrap().clone();
        movement.push('A');
        let mapkey = movement.clone();
        for _ in 0..15 {
            movement = keypad_move(&movement, &directional_keypad_map, true).clone();
        }
        if movement.len() == 0 {
            movement.push('A');
        }
        genmap.insert(mapkey.clone(), movement.len() as i64);
        genmap2.insert(mapkey.clone(), movement.clone());
        println!("{}: {}", mapkey, movement.len());
    }
    
    let test = "140A".to_string();
    let firsgen = keypad_move(&test.clone(), &numeric_keypad_map, false);
    let secgen = keypad_move(&firsgen, &directional_keypad_map, true);

    let mut gentest = keypad_move(&test.clone(), &numeric_keypad_map, false);
    for _ in 0..1 {
        gentest = keypad_move(&gentest, &directional_keypad_map, true);
    }
    println!("Gen: {}", gentest.len());
    let mut demo  = String::new();
    let mut sum     = 0;

    let split = secgen.split("A");
    for part in split {
        let mut part = part.to_string();
        part.push('A');
        let default_part = "A".to_string();
        let thispart = genmap2.get(&part).unwrap_or(&default_part);
        sum += genmap.get(&part).unwrap_or(&1).clone();
        demo.push_str(&thispart.to_string());
    }
    println!("Demo: {}: ", demo.len()-1);
    println!("Sum: {}", sum-1);
    

    let mut total_sum = 0;

    for line in lines {
        let mut movement = keypad_move(&line, &numeric_keypad_map, false);
        println!("Input: {}", line);
        for _ in 0..10 {
            movement = keypad_move(&movement, &directional_keypad_map, true);
        }
        let mut len: i64 = 0;
        let split = movement.split("A");
        for part in split {
            let mut part = part.to_string();
            part.push('A');
            len += genmap.get(&part).unwrap_or(&1).clone() as i64;
        }
        len -= 1;

        let first_three = line.chars().take(3).collect::<String>();
        let numb = first_three.parse::<i64>().unwrap_or(0);
        let result: i64 = len as i64 * numb;

        println!("{} * {} = {}", numb, len, result);

        total_sum += result;
    }

    println!("Total Sum: {}", total_sum);
    Ok(())
    //  169137886514152
    //  145819166348894
    //  145819166348894
    //  360096112011520
    //  360096086018278 too high
    //  145819155818427 too low
    //  145819162083034
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
// 145240 too high.
// 143976 too high.
