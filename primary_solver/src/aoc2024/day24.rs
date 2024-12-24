use tokio::io;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum GateType {
    And,
    Or,
    Xor,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Gate {
    in1: String,
    in2: String,
    op: GateType,
    out: String,
}

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 24, Part A");

    let mut initial_lines = Vec::new();
    let mut gate_lines = Vec::new();
    for line in lines {
        if line.contains("->") {
            gate_lines.push(line.clone());
        } else if line.contains(":") {
            initial_lines.push(line.clone());
        }
    }

    let mut wire_values: HashMap<String, Option<bool>> = HashMap::new();
    for line in &initial_lines {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 {
            let wire_name = parts[0].trim().to_string();
            let bit = match parts[1].trim() {
                "1" => Some(true),
                "0" => Some(false),
                _ => None,
            };
            wire_values.insert(wire_name, bit);
        }
    }

    let mut gates = Vec::new();
    for line in &gate_lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 5 {
            continue;
        }
        let in1 = parts[0].to_string();
        let op = match parts[1].to_uppercase().as_str() {
            "AND" => GateType::And,
            "OR" => GateType::Or,
            "XOR" => GateType::Xor,
            _ => continue,
        };
        let in2 = parts[2].to_string();
        let out = parts[4].to_string();
        wire_values.entry(in1.clone()).or_insert(None);
        wire_values.entry(in2.clone()).or_insert(None);
        wire_values.entry(out.clone()).or_insert(None);
        gates.push(Gate { in1, in2, op, out });
    }

    let mut changed = true;
    while changed {
        changed = false;
        for gate in &gates {
            if wire_values[&gate.out].is_some() {
                continue;
            }
            let left = wire_values[&gate.in1];
            let right = wire_values[&gate.in2];
            if let (Some(a), Some(b)) = (left, right) {
                let r = match gate.op {
                    GateType::And => a && b,
                    GateType::Or => a || b,
                    GateType::Xor => a ^ b,
                };
                wire_values.insert(gate.out.clone(), Some(r));
                changed = true;
            }
        }
    }

    let mut z_wires: Vec<&String> = wire_values.keys().filter(|w| w.starts_with('z')).collect();
    z_wires.sort_by_key(|w| w.trim_start_matches('z').parse::<usize>().unwrap_or(0));
    let mut value: u64 = 0;
    let mut bit_pos = 0;
    for wire in &z_wires {
        if let Some(val) = wire_values[*wire] {
            if val {
                value |= 1 << bit_pos;
            }
        }
        bit_pos += 1;
    }

    println!("{}", value);
    Ok(())
}

fn parse_circuit(lines: &Vec<String>) -> (Vec<Gate>, HashMap<String, Option<bool>>) {
    let mut wire_values: HashMap<String, Option<bool>> = HashMap::new();
    let mut gates = Vec::new();

    for line in lines {
        if line.contains(":") {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                let w = parts[0].trim().to_string();
                let b = match parts[1].trim() {
                    "1" => Some(true),
                    "0" => Some(false),
                    _ => None,
                };
                wire_values.insert(w, b);
            }
        } else if line.contains("->") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                let in1 = parts[0].to_string();
                let op = match parts[1].to_uppercase().as_str() {
                    "AND" => GateType::And,
                    "OR" => GateType::Or,
                    "XOR" => GateType::Xor,
                    _ => continue,
                };
                let in2 = parts[2].to_string();
                let out = parts[4].to_string();
                wire_values.entry(in1.clone()).or_insert(None);
                wire_values.entry(in2.clone()).or_insert(None);
                wire_values.entry(out.clone()).or_insert(None);
                gates.push(Gate { in1, in2, op, out });
            }
        }
    }

    (gates, wire_values)
}

fn simulate_circuit(
    gates: &Vec<Gate>,
    wire_values: &HashMap<String, Option<bool>>,
) -> HashMap<String, Option<bool>> {
    let mut final_values = wire_values.clone();
    let mut changed = true;

    while changed {
        changed = false;
        for gate in gates {
            if final_values[&gate.out].is_some() {
                continue;
            }
            let left = final_values[&gate.in1];
            let right = final_values[&gate.in2];
            if let (Some(a), Some(b)) = (left, right) {
                let r = match gate.op {
                    GateType::And => a && b,
                    GateType::Or => a || b,
                    GateType::Xor => a ^ b,
                };
                final_values.insert(gate.out.clone(), Some(r));
                changed = true;
            }
        }
    }
    final_values
}


fn extract_value(prefix: &str, final_values: &HashMap<String, Option<bool>>) -> u64 {
    let mut wires: Vec<&String> = final_values
        .keys()
        .filter(|k| k.starts_with(prefix))
        .collect();
    wires.sort_by_key(|w| w.trim_start_matches(prefix).parse::<usize>().unwrap_or(0));
    let mut val = 0u64;
    for (i, w) in wires.iter().enumerate() {
        if let Some(true) = final_values[*w] {
            val |= 1 << i;
        }
    }
    val
}


fn build_wire_values(
    x_val: u64,
    y_val: u64,
    max_bits: usize,
    default_wire_values: &HashMap<String, Option<bool>>
) -> HashMap<String, Option<bool>> {
    let mut wv = default_wire_values.clone();
    for i in 0..max_bits {
        let mut wx = format!("x{}", i);
        if i < 10 {
            wx = format!("x0{}", i);
        }
        
        let xbit = ((x_val >> i) & 1) == 1;
        wv.insert(wx, Some(xbit));

        let mut wy = format!("y{}", i);
        if i < 10 {
            wy = format!("y0{}", i);
        }
        let ybit = ((y_val >> i) & 1) == 1;
        wv.insert(wy, Some(ybit));
    }
    wv
}
pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 24, Part B");
    let (gates, _) = parse_circuit(lines);
    let mut visited = HashSet::new();
    let mut result = Vec::new();
    for i in 0..45{
        println!("i: {}", i);
        let mut level_gates = Vec::new();
        for gate in &gates {
            if gate.in1 == format!("x{}", i) || gate.in2 == format!("x{}", i) ||gate.in1 == format!("x0{}", i) || gate.in2 == format!("x0{}", i){
                level_gates.push(gate);
                visited.insert(gate);
                if gate.out.starts_with("z") {
                    if gate.out == "z00"{
                    }
                    else if i <10 {
                        result.push(format!("z0{}", i));
                    } else {
                        result.push(format!("z{}", i));
                    }
                }
            }
        }
        let mut next_step_gates = Vec::new();
        for level_gate in &level_gates {
            for gate in &gates {
                if gate.in1 == level_gate.out || gate.in2 == level_gate.out {
                    next_step_gates.push(gate);
                    visited.insert(gate);
                }
            }
        }
        next_step_gates.sort_by_key(|g| match g.op {
            GateType::And => 0,
            GateType::Or => 1,
            GateType::Xor => 2,
        });
        level_gates.sort_by_key(|g| match g.op {
            GateType::And => 0,
            GateType::Or => 1,
            GateType::Xor => 2,
        });
        println!("x_gates: {:?}", level_gates);
        println!();
        println!("next_step_gates: {:?}", next_step_gates);
        println!();
        let mut xor_gate = None;
        for gate in &next_step_gates {
            if gate.op == GateType::Xor {
                xor_gate = Some(gate);
                break;
            }
        }
        if let Some(gate) = xor_gate {
            if !gate.out.starts_with("z") {
                result.push(gate.out.clone());
                if i <10 {
                    result.push(format!("z0{}", i));
                } else {
                    result.push(format!("z{}", i));
                }
            }
        }
        //look at the inputs from the or gate and make sure they are eqaul to the and gate found on each level
        
    }
    result.sort();
    result.dedup();

    // bfq,bng,fjp,hkh,hmt,z18,z27,z31  ==> line 40
    println!("{}", result.join(","));
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
