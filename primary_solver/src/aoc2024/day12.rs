use std::collections::{HashMap, HashSet, VecDeque};

use tokio::io;

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 12, Part A");

    let mut grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    #[derive(Default, PartialEq, Debug)]
    struct Segment {
        n: bool,
        s: bool,
        e: bool,
        w: bool,
    }

    let mut total_price = 0;

    let mut queue = VecDeque::new();

    let mut bfs = |grid: &mut Vec<Vec<char>>, start: (usize, usize), plant: char| -> i32 {
        println!("Start: {:?}", plant);

        let mut area = 0;
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        visited.insert(start);
        let mut segments: HashMap<(usize, usize), Segment> = HashMap::new();
        let directions = [(-1, 0, 's'), (1, 0, 'n'), (0, -1, 'w'), (0, 1, 'e')];

        queue.push_back(start);

        while let Some((x, y)) = queue.pop_front() {
            let mut seg: Segment = Segment::default();
            grid[x][y] = '.';
            area += 1;
            for &(dx, dy, dir) in &directions {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx < 0 || nx >= rows as isize || ny < 0 || ny >= cols as isize {
                    match dir {
                        'n' => seg.n = true,
                        's' => seg.s = true,
                        'e' => seg.e = true,
                        'w' => seg.w = true,
                        _ => {}
                    }
                } else {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if visited.contains(&(nx, ny)) {
                        continue;
                    } else if grid[nx][ny] == plant {
                        queue.push_back((nx, ny));
                        visited.insert((nx, ny));
                    } else {
                        match dir {
                            'n' => seg.n = true,
                            's' => seg.s = true,
                            'e' => seg.e = true,
                            'w' => seg.w = true,
                            _ => {}
                        }
                    }
                }
            }
            // Add the segment to the map
            segments.insert((x, y), seg);
        }

        let calculate_perimeter = |segments: &HashMap<(usize, usize), Segment>| -> i32 {
            let mut perimeter = 0;

            // Iterate through all segments and count walls
            for (_, seg) in segments {
                if seg.n {
                    perimeter += 1;
                }
                if seg.s {
                    perimeter += 1;
                }
                if seg.e {
                    perimeter += 1;
                }
                if seg.w {
                    perimeter += 1;
                }
            }

            perimeter
        };
        let perimeter = calculate_perimeter(&mut segments);
        let result = area * perimeter;
        // println!("Area: {}, Perimeter: {}", area, perimeter);
        result
    };

    for x in 0..rows {
        for y in 0..cols {
            if grid[x][y] != '.' {
                let plant = grid[x][y];
                let region = bfs(&mut grid, (x, y), plant);
                total_price += region;
            }
            // for row in &grid {
            //     println!("{:?}", row);
            // }
            // println!();
        }
    }

    println!("Total Price of Fencing: {}", total_price);

    Ok(())
}

pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 12, Part A");

    // Parse input into a matrix
    let mut grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    // Struct to represent walls around a cell
    #[derive(Default, PartialEq, Debug)]
    struct Segment {
        n: bool,
        s: bool,
        e: bool,
        w: bool,
    }
    let mut total_price = 0;

    let mut queue = VecDeque::new();

    // BFS to explore a region
    let mut bfs = |grid: &mut Vec<Vec<char>>, start: (usize, usize), plant: char| -> i32 {
        println!("Start: {:?}", plant);

        let mut area = 0;
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        visited.insert(start);
        let mut segments: HashMap<(usize, usize), Segment> = HashMap::new();
        let directions = [(-1, 0, 's'), (1, 0, 'n'), (0, -1, 'w'), (0, 1, 'e')];

        queue.push_back(start);

        while let Some((x, y)) = queue.pop_front() {
            let mut seg: Segment = Segment::default();
            grid[x][y] = '.';
            area += 1;
            for &(dx, dy, dir) in &directions {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx < 0 || nx >= rows as isize || ny < 0 || ny >= cols as isize {
                    match dir {
                        'n' => seg.n = true,
                        's' => seg.s = true,
                        'e' => seg.e = true,
                        'w' => seg.w = true,
                        _ => {}
                    }
                } else {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if visited.contains(&(nx, ny)) {
                        continue;
                    } else if grid[nx][ny] == plant {
                        queue.push_back((nx, ny));
                        visited.insert((nx, ny));
                    } else {
                        match dir {
                            'n' => seg.n = true,
                            's' => seg.s = true,
                            'e' => seg.e = true,
                            'w' => seg.w = true,
                            _ => {}
                        }
                    }
                }
            }
            // Add the segment to the map
            segments.insert((x, y), seg);
        }

        fn no_walls(seg: &mut Segment) -> bool {
            !seg.n && !seg.s && !seg.e && !seg.w
        }

        let calculate_perimeter = |segments: &mut HashMap<(usize, usize), Segment>| -> i32 {
            let mut perimeter = 0;
            while let Some((&(x, y), seg)) = segments.iter().next() {
                //create queue that will be x y and direction
                let mut queue = VecDeque::new();
                //for each direction in seg
                for (dir, is_wall) in vec![('n', seg.n), ('s', seg.s), ('e', seg.e), ('w', seg.w)] {
                    if is_wall {
                        queue.push_back((x, y, dir));
                    }
                }
                //remove the current segment from the hashmap
                segments.remove(&(x, y));
                //now we will iterate through the queue, for each wall we pull out we must find both ends. each piece of the wall we find we will remove that side from the hashmap.
                while let Some((cx, cy, dir)) = queue.pop_front() {
                    match dir {
                        'n' => {
                            // Remove the north wall for the current segment
                            if let Some(current_seg) = segments.get_mut(&(cx, cy)) {
                                current_seg.n = false;
                                if no_walls(current_seg) {
                                    segments.remove(&(cx, cy));
                                }
                            }

                            // Trace horizontally (east-west) for connected north walls
                            let mut run_cy = cy;
                            while run_cy < cols - 1
                                && segments.get(&(cx, run_cy + 1)).map_or(false, |seg| seg.n)
                            {
                                if let Some(east_seg) = segments.get_mut(&(cx, run_cy + 1)) {
                                    east_seg.n = false;
                                    if no_walls(east_seg) {
                                        segments.remove(&(cx, run_cy + 1));
                                    }
                                }
                                run_cy += 1;
                            }

                            run_cy = cy;
                            while run_cy > 0
                                && segments.get(&(cx, run_cy - 1)).map_or(false, |seg| seg.n)
                            {
                                if let Some(west_seg) = segments.get_mut(&(cx, run_cy - 1)) {
                                    west_seg.n = false;
                                    if no_walls(west_seg) {
                                        segments.remove(&(cx, run_cy - 1));
                                    }
                                }
                                run_cy -= 1;
                            }
                            perimeter += 1;
                        }
                        's' => {
                            // Remove the south wall for the current segment
                            if let Some(current_seg) = segments.get_mut(&(cx, cy)) {
                                current_seg.s = false;
                                if no_walls(current_seg) {
                                    segments.remove(&(cx, cy));
                                }
                            }

                            // Trace horizontally (east-west) for connected south walls
                            let mut run_cy = cy;
                            while run_cy < cols - 1
                                && segments.get(&(cx, run_cy + 1)).map_or(false, |seg| seg.s)
                            {
                                if let Some(east_seg) = segments.get_mut(&(cx, run_cy + 1)) {
                                    east_seg.s = false;
                                    if no_walls(east_seg) {
                                        segments.remove(&(cx, run_cy + 1));
                                    }
                                }
                                run_cy += 1;
                            }

                            run_cy = cy;
                            while run_cy > 0
                                && segments.get(&(cx, run_cy - 1)).map_or(false, |seg| seg.s)
                            {
                                if let Some(west_seg) = segments.get_mut(&(cx, run_cy - 1)) {
                                    west_seg.s = false;
                                    if no_walls(west_seg) {
                                        segments.remove(&(cx, run_cy - 1));
                                    }
                                }
                                run_cy -= 1;
                            }

                            perimeter += 1;
                        }
                        'e' => {
                            // Remove the east wall for the current segment
                            if let Some(current_seg) = segments.get_mut(&(cx, cy)) {
                                current_seg.e = false;
                                if no_walls(current_seg) {
                                    segments.remove(&(cx, cy));
                                }
                            }

                            // Trace vertically (north-south) for connected east walls
                            let mut run_cx = cx;
                            while run_cx > 0
                                && segments.get(&(run_cx - 1, cy)).map_or(false, |seg| seg.e)
                            {
                                if let Some(north_seg) = segments.get_mut(&(run_cx - 1, cy)) {
                                    north_seg.e = false;
                                    if no_walls(north_seg) {
                                        segments.remove(&(run_cx - 1, cy));
                                    }
                                }
                                run_cx -= 1;
                            }

                            run_cx = cx;
                            while run_cx < rows - 1
                                && segments.get(&(run_cx + 1, cy)).map_or(false, |seg| seg.e)
                            {
                                if let Some(south_seg) = segments.get_mut(&(run_cx + 1, cy)) {
                                    south_seg.e = false;
                                    if no_walls(south_seg) {
                                        segments.remove(&(run_cx + 1, cy));
                                    }
                                }
                                run_cx += 1;
                            }

                            perimeter += 1;
                        }
                        'w' => {
                            // Remove the west wall for the current segment
                            if let Some(current_seg) = segments.get_mut(&(cx, cy)) {
                                current_seg.w = false;
                                if no_walls(current_seg) {
                                    segments.remove(&(cx, cy));
                                }
                            }

                            // Trace vertically (north-south) for connected west walls
                            let mut run_cx = cx;
                            while run_cx > 0
                                && segments.get(&(run_cx - 1, cy)).map_or(false, |seg| seg.w)
                            {
                                if let Some(north_seg) = segments.get_mut(&(run_cx - 1, cy)) {
                                    north_seg.w = false;
                                    if no_walls(north_seg) {
                                        segments.remove(&(run_cx - 1, cy));
                                    }
                                }
                                run_cx -= 1;
                            }

                            run_cx = cx;
                            while run_cx < rows - 1
                                && segments.get(&(run_cx + 1, cy)).map_or(false, |seg| seg.w)
                            {
                                if let Some(south_seg) = segments.get_mut(&(run_cx + 1, cy)) {
                                    south_seg.w = false;
                                    if no_walls(south_seg) {
                                        segments.remove(&(run_cx + 1, cy));
                                    }
                                }
                                run_cx += 1;
                            }
                            perimeter += 1;
                        }

                        _ => {}
                    }
                }
            }

            perimeter
        };
        let perimeter = calculate_perimeter(&mut segments);
        let result = area * perimeter;
        result
    };

    // Iterate through the grid
    for x in 0..rows {
        for y in 0..cols {
            if grid[x][y] != '.' {
                let plant = grid[x][y];
                let region = bfs(&mut grid, (x, y), plant);
                total_price += region;
            }
        }
    }

    println!("Total Price of Fencing: {}", total_price);

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
