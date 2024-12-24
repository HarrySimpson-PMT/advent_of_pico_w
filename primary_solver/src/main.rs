use std::fs;
use std::path::Path;
mod aoc2024;
mod comms;
use aoc2024::*;
#[allow(unused_imports)]
use comms::pico_sender::send_data_to_pico;

use std::time::Instant;

#[allow(unreachable_code)]
#[tokio::main]
async fn main() {
    let day = 24;

    // let somelines = match get_input_for_puzzle(day) {
    //     Some(lines) => lines,
    //     None => {
    //         println!("Input file not found for puzzle {}", day);
    //         return;
    //     }
    // };
    // let result = send_data_to_pico(&somelines).await;
    // match result {
    //     Ok(_) => println!("Data sent to Pico successfully"),
    //     Err(e) => println!("Error sending data to Pico: {:?}", e),
    // }

    // return;
    
    if let Some(input_lines) = get_input_for_puzzle(day) {
        let start_time = Instant::now();
        let result = day24::solve_b(&input_lines).await;
        let duration = start_time.elapsed();

        println!("Result: {:?}", result);
        println!("Time taken: {:.2?}", duration);
    } else {
        println!("Input file not found for puzzle {}", day);
    }
}

fn get_input_for_puzzle(day: i32) -> Option<Vec<String>> {
    let file_name = format!("day{:02}//file.txt", day);
    let input_path = Path::new("primary_solver\\inputs\\2024").join(file_name);
    print!("{:?}", input_path);
    fs::read_to_string(input_path)
        .ok()
        .map(|content| content.lines().map(String::from).collect())
}
