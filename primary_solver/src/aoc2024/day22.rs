use tokio::io;
use std::collections::HashMap;


fn compute_next_secret(mut secret: u64) -> u64 {
    secret ^= (secret * 64) % 16777216;
    secret %= 16777216;

    secret ^= (secret / 32) % 16777216;
    secret %= 16777216;

    secret ^= (secret * 2048) % 16777216;
    secret %= 16777216;

    secret
}

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 22, Part A");

    let initial_secrets: Vec<u64> = lines
        .iter()
        .filter_map(|line| line.trim().parse::<u64>().ok())
        .collect();

    let total: u64 = initial_secrets
        .iter()
        .map(|&secret| {
            let mut secret = secret;
            for _ in 0..2000 {
                secret = compute_next_secret(secret);
            }
            secret
        })
        .sum();

    println!("The sum of the 2000th secret numbers is: {}", total);

    Ok(())
}


pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    fn next_secret(mut secret: u32) -> u32 {
        let x = (secret as u64).wrapping_mul(64);
        secret ^= x as u32;
        secret &= 0xFFFFFF;

        let y = secret / 32;
        secret ^= y;
        secret &= 0xFFFFFF;

        let z = (secret as u64).wrapping_mul(2048);
        secret ^= z as u32;
        secret &= 0xFFFFFF;

        secret
    }

    fn generate_secrets(initial: u32) -> Vec<u32> {
        let mut secrets = Vec::with_capacity(2001);
        secrets.push(initial);
        for i in 1..=2000 {
            let s = next_secret(secrets[i - 1]);
            secrets.push(s);
        }
        secrets
    }

    fn generate_prices(secrets: &[u32]) -> Vec<u32> {
        secrets.iter().map(|&s| s % 10).collect()
    }

    fn generate_changes(prices: &[u32]) -> Vec<i32> {
        let mut changes = Vec::with_capacity(prices.len() - 1);
        for i in 0..prices.len() - 1 {
            changes.push(prices[i + 1] as i32 - prices[i] as i32);
        }
        changes
    }

    let mut pattern_sum: HashMap<(i32, i32, i32, i32), u64> = HashMap::new();

    for line in lines {
        let initial = line.parse::<u32>().unwrap();

        let secrets = generate_secrets(initial);
        let prices = generate_prices(&secrets);
        let changes = generate_changes(&prices);

        let mut buyer_map = HashMap::<(i32, i32, i32, i32), u32>::new();

        for i in 0..=(changes.len() - 4) {
            let c1 = changes[i];
            let c2 = changes[i + 1];
            let c3 = changes[i + 2];
            let c4 = changes[i + 3];
            let pattern = (c1, c2, c3, c4);
            
            if !buyer_map.contains_key(&pattern) {
                buyer_map.insert(pattern, prices[i + 4]);
            }
        }

        for (pattern, earliest_price) in buyer_map {
            *pattern_sum.entry(pattern).or_insert(0) += earliest_price as u64;
        }
    }

    let best = pattern_sum.values().max().copied().unwrap_or(0);

    println!();
    println!("{}", best);

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
