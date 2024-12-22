use std::collections::HashMap;
use std::{default, string};
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
    //compare the two
    println!("{}: {}", north_west_a_gen2.len(), north_west_b_gen2.len());

    let north_east_a = "^>A".to_string();
    let north_east_b = ">^A".to_string();
    let north_east_a_gen1 = keypad_move(&north_east_a, &directional_keypad_map, true);
    let north_east_b_gen1 = keypad_move(&north_east_b, &directional_keypad_map, true);
    let north_east_a_gen2 = keypad_move(&north_east_a_gen1, &directional_keypad_map, true);
    let north_east_b_gen2 = keypad_move(&north_east_b_gen1, &directional_keypad_map, true);
    //compare the two
    println!("{}: {}", north_east_a_gen2.len(), north_east_b_gen2.len());

    let south_west_a = "v<A".to_string();
    let south_west_b = "<vA".to_string();
    let south_west_a_gen1 = keypad_move(&south_west_a, &directional_keypad_map, true);
    let south_west_b_gen1 = keypad_move(&south_west_b, &directional_keypad_map, true);
    let south_west_a_gen2 = keypad_move(&south_west_a_gen1, &directional_keypad_map, true);
    let south_west_b_gen2 = keypad_move(&south_west_b_gen1, &directional_keypad_map, true);
    //compare the two
    println!("{}: {}", south_west_a_gen2.len(), south_west_b_gen2.len());

    let south_east_a = "v>A".to_string();
    let south_east_b = ">vA".to_string();
    let south_east_a_gen1 = keypad_move(&south_east_a, &directional_keypad_map, true);
    let south_east_b_gen1 = keypad_move(&south_east_b, &directional_keypad_map, true);
    let south_east_a_gen2 = keypad_move(&south_east_a_gen1, &directional_keypad_map, true);
    let south_east_b_gen2 = keypad_move(&south_east_b_gen1, &directional_keypad_map, true);
    //compare the two
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
        //print line: third
        println!("{}: {}", line, third);
        //print numb * len = result
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

    // Apply horizontal movement first
    if horizontal_moves < 0 {
        movement.push_str(&"<".repeat(-horizontal_moves as usize));
    } else if horizontal_moves > 0 {
        movement.push_str(&">".repeat(horizontal_moves as usize));
    }

    // Apply vertical movement
    if vertical_moves < 0 {
        movement.push_str(&"^".repeat(-vertical_moves as usize));
    } else if vertical_moves > 0 {
        movement.push_str(&"v".repeat(vertical_moves as usize));
    }

    let mut test_pos = start;

    if horizontal_moves < 0 {
        test_pos.col += horizontal_moves;
        if is_directional {
            if test_pos == Position::new(0, 0) {
                movement = sort_movement(&movement, true); // Invert sorting
            } else {
                movement = sort_movement(&movement, false);
            }
        } else {
            if test_pos == Position::new(3, 0) {
                movement = sort_movement(&movement, true); // Invert sorting
            } else {
                movement = sort_movement(&movement, false);
            }
        }
    } else if vertical_moves > 0 {
        test_pos.row += vertical_moves;
        if is_directional {
            if test_pos == Position::new(0, 0) {
                movement = sort_movement(&movement, true); // Invert sorting
            } else {
                movement = sort_movement(&movement, false);
            }
        } else {
            if test_pos == Position::new(3, 0) {
                movement = sort_movement(&movement, true); // Invert sorting
            } else {
                movement = sort_movement(&movement, false);
            }
        }
    } else if horizontal_moves > 0 {
        test_pos.col += horizontal_moves;

        if is_directional {
            if test_pos == Position::new(0, 0) {
                movement = sort_movement(&movement, true); // Invert sorting
            } else {
                movement = sort_movement(&movement, false);
            }
        } else {
            if test_pos == Position::new(3, 0) {
                movement = sort_movement(&movement, true); // Invert sorting
            } else {
                movement = sort_movement(&movement, false);
            }
        }
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
    let input = lines.join("\n");
    let result = calculate_combo_complexities::<25>(&input);
    print!("Result: {}", result);
//     return Ok(());

// //allow unreachable code
    // #[allow(unreachable_code)]    
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
    let thirdgen = keypad_move(&secgen, &directional_keypad_map, true);
    let fourthgen = keypad_move(&thirdgen, &directional_keypad_map, true);
    let fifthgen = keypad_move(&fourthgen, &directional_keypad_map, true);
    let sixthgen = keypad_move(&fifthgen, &directional_keypad_map, true);
    let seventhgen = keypad_move(&sixthgen, &directional_keypad_map, true);
    let eigthgen = keypad_move(&seventhgen, &directional_keypad_map, true);
    let ninethgen = keypad_move(&eigthgen, &directional_keypad_map, true);
    let tenthgen = keypad_move(&ninethgen, &directional_keypad_map, true);

    println!("Test: {}", thirdgen.len());
    let mut gentest = keypad_move(&test.clone(), &numeric_keypad_map, false);
    for i in 0..1 {
        gentest = keypad_move(&gentest, &directional_keypad_map, true);
    }
    println!("Gen: {}", gentest.len());
    //demo new string
    let mut demo  = String::new();
    let mut sum     = 0;

    let mut split = secgen.split("A");
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
        //iterate through the movement and split after every A keep A to the left, then look up the value in the genmap
        let mut split = movement.split("A");
        for part in split {
            let mut part = part.to_string();
            part.push('A');
            let default_part = "A".to_string();
            let thispart = genmap2.get(&part).unwrap_or(&default_part);
            len += genmap.get(&part).unwrap_or(&1).clone() as i64;
        }
        //minus 1 len
        len -= 1;

        let first_three = line.chars().take(3).collect::<String>();
        let numb = first_three.parse::<i64>().unwrap_or(0);
        let result: i64 = len as i64 * numb;

        // Print the final result
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

#[derive(Debug, Clone, Copy)]
enum DirectionKey {
    Up = 0,
    Activate = 1,
    Left = 2,
    Down = 3,
    Right = 4,
}

fn calc_level_costs(previous_costs: &[u64], new_costs: &mut [u64], paths: &[Vec<Vec<DirectionKey>>]) {
    for (paths, new_cost) in paths.iter().zip(new_costs) {
        *new_cost = paths.iter().map(|path| {
            // Sum up the costs of going from each button to the next one and pressing it, starting from Activate
            let mut pos = DirectionKey::Activate;
            path.iter().map(|&new_pos| {
                let cost = previous_costs[pos as usize * 5 + new_pos as usize];
                pos = new_pos;
                cost
            }).sum()
        }).min().unwrap()
    }
}

fn get_paths<const HOLE_Y: u8>(paths: &mut Vec<Vec<DirectionKey>>, key_positions: &[[u8; 2]], start: usize, end: usize) {
    let [start_x, start_y] = key_positions[start];
    let [end_x, end_y] = key_positions[end];

    if !(start_x == 0 && end_y == HOLE_Y) {
        // Start by going vertically and then horizontally
        // This must not be done if we start on the left button and go to the top row, as that would make us pass
        // over an empty space.
        let mut path = Vec::new();
        if start_y < end_y {
            path.extend((start_y..end_y).map(|_| DirectionKey::Down));
        } else if start_y > end_y {
            path.extend((end_y..start_y).map(|_| DirectionKey::Up));
        }
        if start_x < end_x {
            path.extend((start_x..end_x).map(|_| DirectionKey::Right));
        } else if start_x > end_x {
            path.extend((end_x..start_x).map(|_| DirectionKey::Left));
        }
        // We always need to end with Activate, so we actually press the button we go to.
        path.push(DirectionKey::Activate);
        paths.push(path);
    }

    if start_x != end_x && start_y != end_y && !(start_y == HOLE_Y && end_x == 0) {
        // If we need to both vertically and horizontally, we can also do it by going horizontally first.
        // This must not be done if we end on the left button, as that would make us pass over an empty space.
        let mut path = Vec::new();
        if start_x < end_x {
            path.extend((start_x..end_x).map(|_| DirectionKey::Right));
        } else if start_x > end_x {
            path.extend((end_x..start_x).map(|_| DirectionKey::Left));
        }
        if start_y < end_y {
            path.extend((start_y..end_y).map(|_| DirectionKey::Down));
        } else if start_y > end_y {
            path.extend((end_y..start_y).map(|_| DirectionKey::Up));
        }
        // We always need to end with Activate, so we actually press the button we go to.
        path.push(DirectionKey::Activate);
        paths.push(path);
    }

    // It is never worth zigzagging, as such paths can be reduced into a non-zigzagging path just by duplicating
    // some presses while eliminating others, to get a path that takes fewer presses in total.
}

fn calc_directional_key_costs<const ROBOT_KEYPADS: u8>() -> Vec<u64> {
    // Where each key is located on the directional keypad
    let direction_key_positions = [
        [1, 0],
        [2, 0],
        [0, 1],
        [1, 1],
        [2, 1],
    ];

    // Possible button inputs required to get the robot at the next level to press any button from any starting position
    let direction_key_paths: Vec<_> = (0..(5 * 5)).map(|i| {
        let mut paths = Vec::new();
        let start = i / 5;
        let end = i % 5;
        get_paths::<0>(&mut paths, &direction_key_positions, start, end);
        paths
    }).collect();

    // How many button presses it takes to get to any button from any other button and then press it
    let mut path_costs: Vec<u64> = direction_key_paths.iter().map(|paths| {
        paths.iter().map(|path| path.len() as u64).min().unwrap()
    }).collect();

    let mut new_costs = vec![0; 5 * 5];
    for _ in 0..ROBOT_KEYPADS - 1 {
        calc_level_costs(&path_costs, &mut new_costs, &direction_key_paths);
        std::mem::swap(&mut path_costs, &mut new_costs);
    }

    path_costs
}

fn calculate_combo_complexities<const ROBOT_KEYPADS: u8>(input: &str) -> u64 {
    let directional_key_costs = calc_directional_key_costs::<ROBOT_KEYPADS>();

    // Where each key is located on the numeric keypad.
    // Elements represent positions of 0-9 followed by A
    let numeric_key_positions = [
        [1, 3],
        [0, 2],
        [1, 2],
        [2, 2],
        [0, 1],
        [1, 1],
        [2, 1],
        [0, 0],
        [1, 0],
        [2, 0],
        [2, 3],
    ];

    let mut paths = Vec::new();
    input.lines().map(|combo| {
        let number: u64 = combo.split('A').next().unwrap().parse().unwrap();

        let mut pos = numeric_key_positions.len() - 1;
        combo.chars().map(|key| {
            let new_pos = match key {
                '0'..='9' => key as usize - '0' as usize,
                'A' => 10,
                _ => panic!("Invalid character: {}", key),
            };
            get_paths::<3>(&mut paths, &numeric_key_positions, pos, new_pos);
            let cost: u64 = paths.iter().map(|path| {
                let mut pos = DirectionKey::Activate;
                path.iter().map(|&new_pos| {
                    let cost = directional_key_costs[pos as usize * 5 + new_pos as usize];
                    pos = new_pos;
                    cost
                }).sum()
            }).min().unwrap();
            pos = new_pos;
            paths.clear();
            cost
        }).sum::<u64>() * number
    }).sum()
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
