use std::{collections::{HashMap, HashSet}, fs};

const FILENAME: &str = "./inputs/input.txt";

const START: &str = "AAA";
const END: &str = "ZZZ";

fn main() {
    let (
        instructions,
        nodes,
        node_mapping
    ) = parse_input();

    let mut location = START.to_string();
    let mut step_idx = 0;
    loop {
        if location == END.to_string() {
            break;
        }

        let instruct = instructions[step_idx % instructions.len()];
        let next_node = node_mapping.get(&location).unwrap()[left_or_right(instruct)].clone();
        location = next_node;
        step_idx += 1;
    }

    println!("Part 1: {}", step_idx);


    let mut locations = nodes.iter().filter(|node| node.ends_with("A")).collect::<Vec<_>>();
    let end_nodes = nodes.iter().filter(|node| node.ends_with("Z")).collect::<Vec<_>>();

    println!("{:?}", locations);
    println!("{:?}", end_nodes);

    let mut step_idx_2 = 0;

    let mut min_steps_to_end = vec![None; locations.len()];
    loop {
        if min_steps_to_end.iter().all(|step_count| step_count.is_some()) {
            break;
        }

        let mut new_locations = vec![];

        for (loc_idx, loc) in locations.iter().enumerate() {
            if end_nodes.contains(loc) && min_steps_to_end[loc_idx].is_none() {
                min_steps_to_end[loc_idx] = Some(step_idx_2);
            }

            let instruct = instructions[step_idx_2 % instructions.len()];
            let next_node = &node_mapping.get(*loc).unwrap()[left_or_right(instruct)];
            new_locations.push(next_node);
        }
        locations = new_locations;

        step_idx_2 += 1;
    }

    let min_steps = min_steps_to_end.iter().map(|v| v.unwrap() as u128).collect::<Vec<u128>>();

    println!("Min steps: {:?}", min_steps);
    println!("Part 2: {}", lcm(&min_steps));
}

fn lcm (values: &Vec<u128>) -> u128 {
    let gcd = gcd(values);
    println!("GCD is: {}", gcd);
    let prod = values.iter().map(|v| v / gcd).fold(1, |acc, val| val * acc);
    return prod * gcd;
}

fn gcd (values: &Vec<u128>) -> u128 {
    let min_val = values.iter().min().unwrap();

    for i in (1..(*min_val + 1)).rev() {
        if values.iter().all(|v| v % i == 0) {
            return i;
        }
    }
    
    return 1;
}

fn left_or_right (dir: char) -> usize {
    match dir {
        'L' => 0,
        'R' => 1,
        _ => panic!()
    }
}

fn parse_input () -> (Vec<char>, Vec<String>, HashMap<String, [String; 2]>) {
    let mut instructions = vec![];
    let mut nodes = vec![];
    let mut node_mapping = HashMap::new();

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        if line.len() == 0 {
            continue;
        }

        if !line.contains("=") {
            instructions = line.chars().collect::<Vec<char>>();
            continue;
        }

        let sections = line.split(" = ").collect::<Vec<_>>();
        let node_id = sections[0].to_string();
        nodes.push(node_id.clone());

        let leaf_string = sections[1].replace("(", "").replace(")", "");
        let leaves = leaf_string.split(", ").collect::<Vec<_>>();

        node_mapping.insert(node_id.clone(), [leaves[0].to_string(), leaves[1].to_string()]);
    }

    return (instructions, nodes, node_mapping);
}