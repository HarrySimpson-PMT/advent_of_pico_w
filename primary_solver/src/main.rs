#[allow(dead_code)]
use std::fs;
use std::path::Path;
mod aoc2024;

use aoc2024::*;

#[derive(Debug)]
enum Puzzle {
    Day01A,
    Day01B,
    Day02A,
    Day02B,
    Day03A,
    Day03B,
    Day04A,
    Day04B,
    Day05A,
    Day05B,
}

impl Puzzle {
    #[allow(dead_code)]
    pub fn from_day_part(day: u8, part: char) -> Self {
        match (day, part) {
            (1, 'A') => Puzzle::Day01A,
            (1, 'B') => Puzzle::Day01B,
            (2, 'A') => Puzzle::Day02A,
            (2, 'B') => Puzzle::Day02B,
            (3, 'A') => Puzzle::Day03A,
            (3, 'B') => Puzzle::Day03B,
            (4, 'A') => Puzzle::Day04A,
            (4, 'B') => Puzzle::Day04B,       
            (5, 'A') => Puzzle::Day05A,
            (5, 'B') => Puzzle::Day05B,
            _ => panic!("Invalid day or part"),
        }
    }

    pub fn to_day_part(&self) -> (u8, char) {
        match self {
            Puzzle::Day01A => (1, 'A'),
            Puzzle::Day01B => (1, 'B'),
            Puzzle::Day02A => (2, 'A'),
            Puzzle::Day02B => (2, 'B'),
            Puzzle::Day03A => (3, 'A'),
            Puzzle::Day03B => (3, 'B'),
            Puzzle::Day04A => (4, 'A'),
            Puzzle::Day04B => (4, 'B'),
            Puzzle::Day05A => (5, 'A'),
            Puzzle::Day05B => (5, 'B'),
        }
    }
}
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

#[tokio::main]
async fn main() {
    let selected_puzzle = Puzzle::Day04B;

    if let Some(input_lines) = get_input_for_puzzle(&selected_puzzle) {
        println!("Number of lines: {}", input_lines.len());

        match selected_puzzle {
            Puzzle::Day01A => {
                if let Err(e) = day01::solve_a(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day01B => {
                if let Err(e) = day01::solve_b(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day02A => {
                if let Err(e) = day02::solve_a(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day02B => {
                if let Err(e) = day02::solve_b(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day03A => {
                if let Err(e) = day03::solve_a(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day03B => {
                if let Err(e) = day03::solve_b(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day04A => {
                if let Err(e) = day04::solve_a(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day04B => {
                if let Err(e) = day04::solve_b(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day05A => {
                if let Err(e) = day05::solve_a(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
            Puzzle::Day05B => {
                if let Err(e) = day05::solve_b(&input_lines).await {
                    eprintln!("Error: {}", e);
                }
            }
        }
    } else {
        println!("Input file not found for puzzle: {:?}", selected_puzzle);
    }
}

fn get_input_for_puzzle(puzzle: &Puzzle) -> Option<Vec<String>> {
    let (day, _part) = puzzle.to_day_part();

    let file_name = format!("day{:02}//file.txt", day);
    let input_path = Path::new("primary_solver\\inputs\\2024").join(file_name);
    print!("{:?}", input_path);

    fs::read_to_string(input_path)
        .ok()
        .map(|content| content.lines().map(String::from).collect())
}

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

pub async fn send_data_to_pico(lines: Vec<String>) -> io::Result<()> {
    let host = "10.0.0.139";
    let port = 1234;
    let address = format!("{}:{}", host, port);
    println!(
        "Connecting to {}:{} to send {} lines",
        host,
        port,
        lines.len()
    );

    let mut stream = match timeout(Duration::from_secs(5), TcpStream::connect(&address)).await {
        Ok(Ok(stream)) => {
            println!("Successfully connected to the server!");
            stream
        }
        Ok(Err(e)) => {
            eprintln!("Connection failed: {}", e);
            return Err(e);
        }
        Err(_) => {
            eprintln!("Connection timed out.");
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "Connection timed out",
            ));
        }
    };

    use tokio::time::{sleep, Duration};

    async fn read_ack() {
        sleep(Duration::from_millis(80)).await;
    }

    println!("Sending 'Start'");
    stream.write_all(b"Start\r\n").await?;
    stream.flush().await?;
    read_ack().await;

    let line_count = lines.len();
    println!("Sending line count: {}", line_count);
    stream
        .write_all(format!("{}\r\n", line_count).as_bytes())
        .await?;
    stream.flush().await?;
    read_ack().await;

    for line in &lines {
        println!("Sending line: {}", line);
        stream.write_all(format!("{}\r\n", line).as_bytes()).await?;
        stream.flush().await?;
        read_ack().await;
    }

    println!("Sending 'GO'");
    stream.write_all(b"GO\r\n").await?;
    stream.flush().await?;
    read_ack().await;

    let mut buffer = [0; 1024];
    println!("Waiting for final response...");
    let n = stream.read(&mut buffer).await?;
    println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));

    println!("Data sent successfully!");
    Ok(())
}
