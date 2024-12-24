use std::collections::{HashMap, HashSet};
use tokio::io;

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 23, Part A");

    let mut adjacency: HashMap<String, HashSet<String>> = HashMap::new();
    for line in lines {
        let parts: Vec<&str> = line.split('-').collect();
        let (a, b) = (parts[0].to_string(), parts[1].to_string());
        adjacency.entry(a.clone()).or_default().insert(b.clone());
        adjacency.entry(b).or_default().insert(a);
    }

    let mut cliques = Vec::new();

    for (node, neighbors) in &adjacency {
        let neighbors_vec: Vec<&String> = neighbors.iter().collect();
        for i in 0..neighbors_vec.len() {
            for j in i + 1..neighbors_vec.len() {
                let a = neighbors_vec[i];
                let b = neighbors_vec[j];
                if adjacency.get(a).unwrap().contains(b) {
                    let mut clique = vec![node.clone(), a.clone(), b.clone()];
                    clique.sort(); 
                    cliques.push(clique);
                }
            }
        }
    }

    let mut unique_cliques: HashSet<Vec<String>> = HashSet::new();
    for clique in cliques {
        unique_cliques.insert(clique);
    }

    let t_cliques: Vec<Vec<String>> = unique_cliques
        .into_iter()
        .filter(|clique| clique.iter().any(|name| name.starts_with('t')))
        .collect();

    println!("Number of valid cliques: {}", t_cliques.len());

    Ok(())
}

pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 23, Part B");

    let mut adjacency: HashMap<String, HashSet<String>> = HashMap::new();
    for line in lines {
        let parts: Vec<&str> = line.split('-').collect();
        let (a, b) = (parts[0].to_string(), parts[1].to_string());
        adjacency.entry(a.clone()).or_default().insert(b.clone());
        adjacency.entry(b).or_default().insert(a);
    }

    let mut visited = HashSet::new();
    let mut largest_clique: Vec<String> = Vec::new();

    for node in adjacency.keys() {
        if visited.contains(node) {
            continue;
        }

        let mut stack = vec![node.clone()];
        let mut component = HashSet::new();

        while let Some(current) = stack.pop() {
            if !visited.insert(current.clone()) {
                continue;
            }
            component.insert(current.clone());
            for neighbor in &adjacency[&current] {
                if !visited.contains(neighbor) {
                    stack.push(neighbor.clone());
                }
            }
        }

        let candidate = find_largest_clique_in_component(&adjacency, &component);
        if candidate.len() > largest_clique.len() {
            largest_clique = candidate;
        }
    }

    largest_clique.sort();
    let password = largest_clique.join(",");
    println!("LAN party password: {}", password);

    Ok(())
}
fn find_largest_clique_in_component(
    adj: &HashMap<String, HashSet<String>>,
    comp_nodes: &HashSet<String>,
) -> Vec<String> {
    let mut r = HashSet::new();
    let mut p = comp_nodes.clone();
    let mut x = HashSet::new();
    let mut all_maximal: Vec<Vec<String>> = Vec::new();

    bron_kerbosch(adj, &mut r, &mut p, &mut x, &mut all_maximal);

    all_maximal.into_iter().max_by_key(|c| c.len()).unwrap_or_default()
}

fn bron_kerbosch(
    adj: &HashMap<String, HashSet<String>>,
    r: &mut HashSet<String>,
    p: &mut HashSet<String>,
    x: &mut HashSet<String>,
    maximal_cliques: &mut Vec<Vec<String>>,
) {
    if p.is_empty() && x.is_empty() {
        let mut clique: Vec<_> = r.iter().cloned().collect();
        clique.sort();
        maximal_cliques.push(clique);
        return;
    }

    let pivot = p.union(x).next().cloned();

    if let Some(u) = pivot {
        let neighbors_u = &adj[&u];
        let to_visit: Vec<String> = p.difference(neighbors_u).cloned().collect();
        for v in to_visit {
            r.insert(v.clone());
            p.remove(&v);
            
            let nbrs_v = &adj[&v];
            let mut p_next: HashSet<String> = p.intersection(nbrs_v).cloned().collect();
            let mut x_next: HashSet<String> = x.intersection(nbrs_v).cloned().collect();

            bron_kerbosch(adj, r, &mut p_next, &mut x_next, maximal_cliques);

            r.remove(&v);
            x.insert(v);
        }
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
