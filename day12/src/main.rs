use std::{collections::HashMap, fs};

const FILENAME: &str = "./inputs/input.txt";

const OPERATIONAL_CHAR: char = '.';
const DAMAGED_CHAR: char = '#';
const UNKNOWN_CHAR: char = '?';
const CONDITION_MAPPING: [char; 2] = [OPERATIONAL_CHAR, DAMAGED_CHAR];

const VERBOSE: bool = false;

fn main() {
    let spring_rows = parse_input();
    println!("Rows: {}", spring_rows.len());

    let mut known_solutions = HashMap::new();

    let mut total = 0;
    let mut unfolded_total = 0;

    for (idx, row) in spring_rows.iter().enumerate() {
         println!("------{}-----", idx);
        // println!("{:?}", row);
        // let valid_permutations = brute_force(row);
        let valid_permutations = solve(&row, &mut known_solutions);
        total += valid_permutations;

        let unfolded_row = row.repeat(5);
        // unfolded_total += solve(&unfolded_row, &mut known_solutions);
        if row.conditions.contains(&OPERATIONAL_CHAR) {
            let count = split_solve(&unfolded_row);
            println!("{}", count);
            unfolded_total += count;
        } else {
            let count = solve(&unfolded_row, &mut known_solutions);
            println!("{}", count);
            unfolded_total += count;
        }
        // println!("{}", valid_permutations);
    }

    println!("Part 1: {}", total);
    println!("Part 2: {}", unfolded_total);


}

fn split_solve (row: &SpringRow) -> usize {
    let condition_sections = row.conditions.split(|c| *c == OPERATIONAL_CHAR).collect::<Vec<&[char]>>();
    
    let mut section_configs = vec![];

    for section in condition_sections.iter() {
        let possible_configs = possible_configurations(&section.to_vec());
        section_configs.push(possible_configs);
    }

    return valid_configurations(&row.damaged_blocks, &section_configs, 0);

    // return 0;
}

fn valid_configurations (damaged_blocks: &Vec<usize>, configs: &Vec<HashMap<Vec<usize>, usize>>, section_idx: usize) -> usize {
    if configs.len() == section_idx {
        return 1;
    }

    let section_config = &configs[section_idx];

    let mut total = 0;
    
    for (config, count) in section_config.iter() {
        if damaged_blocks.starts_with(config) {
            total += count * valid_configurations(&damaged_blocks[config.len()..].to_vec(), configs, section_idx + 1);
        }
    }

    return total;
}

fn possible_configurations (conditions: &Vec<char>) -> HashMap<Vec<usize>, usize> {
    let mut configurations = HashMap::new();

    let n_unknowns = conditions.iter().filter(|s| **s == UNKNOWN_CHAR).count();
    let unknown_indices = find_unknowns(conditions);
    // println!("Unknown indices: {:?}", unknown_indices);

    // if n_unknowns == 0 {
    //     configurations.insert(condition_representation(&conditions), 1);
    // }

    for i in 0..2_usize.pow(n_unknowns as u32) {
        let conditions = fill_unknowns(conditions, i, &unknown_indices);
        let block_representation = condition_representation(&conditions);
        if configurations.contains_key(&block_representation) {
            *configurations.get_mut(&block_representation).unwrap() += 1;
        } else {
            configurations.insert(block_representation, 1);
        }
    }

    // println!("Possible configurations for permutation: {:?}", conditions);
    // println!("{:?}", configurations);

    return configurations;
}

fn fill_all_certain_unknowns(row: &SpringRow) -> SpringRow {
    let mut filled_row = row.clone();

    loop {
        let unknown_indices: Vec<usize> = find_unknowns(&filled_row.conditions);
        let new_filled_row = fill_certain_unknowns(&filled_row, &unknown_indices, 0);

        if filled_row.conditions == new_filled_row.conditions {
            return filled_row;
        }
        filled_row = new_filled_row
    }
}

fn fill_certain_unknowns (row: &SpringRow, unknown_indices: &Vec<usize>, unknown_idx: usize) -> SpringRow {
    if unknown_idx == unknown_indices.len() {
        return row.clone();
    }

    let with_damaged = fill_unknown(&row.conditions, unknown_indices[unknown_idx], DAMAGED_CHAR);
    let with_operational = fill_unknown(&row.conditions, unknown_indices[unknown_idx], OPERATIONAL_CHAR);

    let valid_with_damaged = check_potentially_valid(&with_damaged, &row.damaged_blocks);
    let valid_with_operational = check_potentially_valid(&with_operational, &row.damaged_blocks);

    let next_conditions;

    if valid_with_damaged && !valid_with_operational {
        next_conditions = with_damaged;
    } else if !valid_with_damaged && valid_with_operational {
        next_conditions = with_operational;
    } else {
        next_conditions = (*row.conditions).to_vec();
    }

    let next_row = SpringRow {
        conditions: next_conditions,
        damaged_blocks: (*row.damaged_blocks).to_vec()
    };

    return fill_certain_unknowns(&next_row, unknown_indices, unknown_idx + 1)
}

fn stripped_conditions (conditions: &Vec<char>, unknown_indices: &Vec<usize>) -> Vec<char> {
    if unknown_indices.len() == 0 {
        return conditions.clone();
    }

    return conditions[unknown_indices[0]..].to_vec();
}

// this is still a bit slow but whatever
fn solve (row: &SpringRow, known_solutions: &mut HashMap<Vec<char>, usize>) -> usize {
    let partially_filled_row = row;
    // let partially_filled_row = fill_all_certain_unknowns(&row);

    if VERBOSE { println!("Solving for: {:?}", partially_filled_row) };
    let unknown_indices: Vec<usize> = find_unknowns(&partially_filled_row.conditions);

    // if known_solutions.contains_key(&stripped_conditions(&row.conditions, &unknown_indices)) {
    //     return *known_solutions.get(&stripped_conditions(&row.conditions, &unknown_indices)).unwrap();
    // }

    if unknown_indices.len() == 0 {
        if VERBOSE { println!("No unknowns to fill, checking if solution is valid...") };
        if check_valid(&partially_filled_row.conditions, &partially_filled_row.damaged_blocks) {
            if VERBOSE { println!("Valid") };
            return 1;
        } else {
            if VERBOSE { println!("Invalid") };
            return 0;
        }
    }

    let mut total = 0;
    let with_damaged = fill_unknown(&partially_filled_row.conditions, unknown_indices[0], DAMAGED_CHAR);
    let with_operational = fill_unknown(&partially_filled_row.conditions, unknown_indices[0], OPERATIONAL_CHAR);

    if VERBOSE { 
    println!("Checking potential validity of damaged & operational variants:");
    println!("Damaged: {:?}", with_damaged);
    println!("Operational: {:?}", with_operational);
    }
    let mut new_unknown_indices = unknown_indices.clone();
    new_unknown_indices.remove(0);

    if check_potentially_valid(&with_damaged, &partially_filled_row.damaged_blocks) {
        if VERBOSE { println!("damaged variant is potentially valid...") };
        let hash_key = stripped_conditions(&with_damaged, &new_unknown_indices);
        let new_row = SpringRow {
            conditions: with_damaged,
            damaged_blocks: (*partially_filled_row.damaged_blocks).to_vec()
        };
        let n_solutions = solve(&new_row, known_solutions);
        known_solutions.insert(hash_key, n_solutions);
        total += n_solutions;
    }
    if check_potentially_valid(&with_operational, &partially_filled_row.damaged_blocks) {
        if VERBOSE { println!("oeprational variant is potentially valid...") };
        let hash_key = stripped_conditions(&with_operational, &new_unknown_indices);
        let new_row = SpringRow {
            conditions: with_operational,
            damaged_blocks: (*partially_filled_row.damaged_blocks).to_vec()
        };
        let n_solutions = solve(&new_row, known_solutions);
        known_solutions.insert(hash_key, n_solutions);
        total += n_solutions;
    }

    return total
}

fn fill_unknown (conditions: &Vec<char>, unknown_idx: usize, unknown_val: char) -> Vec<char> {
    let mut new_conditions = conditions.clone();
    new_conditions[unknown_idx] = unknown_val;
    return new_conditions
}

fn find_unknowns (conditions: &Vec<char>) -> Vec<usize> {
    return conditions.iter().enumerate().filter(|(idx, s)| **s == UNKNOWN_CHAR).map(|(idx, s)| idx).collect::<Vec<_>>();
}

fn brute_force (row: &SpringRow) -> usize {
    let mut valid_combinations = 0;

    let n_unknowns = row.conditions.iter().filter(|s| **s == UNKNOWN_CHAR).count();
    let unknown_indices = find_unknowns(&row.conditions);
    // println!("Unknown indices: {:?}", unknown_indices);

    for i in 0..2_usize.pow(n_unknowns as u32) {
        // let partial_conditions = partial_fill_unknowns(&row.conditions, i, &unknown_indices, partial_unknowns);
        // if !check_potentially_valid(&partial_conditions,  &row.damaged_blocks) {

        // }


        let conditions = fill_unknowns(&row.conditions, i, &unknown_indices);
        if check_valid(&conditions, &row.damaged_blocks) {
            valid_combinations += 1;
        }
    }

    return valid_combinations;
}

fn check_potentially_valid (conditions: &Vec<char>, damaged_blocks: &Vec<usize>) -> bool {
    let mut block_representation: Vec<usize> = vec![0];
    let mut block_idx = 0;
    // println!("Checking: {:?}", conditions);

    for c in conditions {
        if *c == UNKNOWN_CHAR {
            // return or something
            block_representation.pop();
            return damaged_blocks.starts_with(&block_representation);
        }

        if *c == DAMAGED_CHAR {
            *block_representation.last_mut().unwrap() += 1;
            continue;
        }

        if *block_representation.last().unwrap() == 0 {
            // condition is operational, in a string of operational gears
            // 
            continue;
        }

        
        if block_idx >= damaged_blocks.len() || *block_representation.last().unwrap() != damaged_blocks[block_idx] {
            // condition is operational, end of block of damaged gears
            // number of damaged gears doesn't match expected number
            // not valid
            return false
        }

        // condition is operational
        // start new count of damaged gears
        // will increment when next see a damaged gear
        block_idx += 1;
        block_representation.push(0);
    }
    if *block_representation.last().unwrap() == 0 {
        block_representation.pop();
    }
    // println!("Representation: {:?}", block_representation);

    return block_representation == *damaged_blocks;
}

fn partial_fill_unknowns (conditions: &Vec<char>, mask: usize, unknown_indices: &Vec<usize>, partial_unknowns: usize) -> Vec<char> {
    let condition_mapping = [OPERATIONAL_CHAR, DAMAGED_CHAR];
    let mut condition_permutation = conditions.clone();

    for idx in 0..partial_unknowns {
        let index = unknown_indices[idx];
        // println!("mask: {}, idx: {}, n_indices: {}", mask, idx, n_indices);
        let condition_val = (mask >> idx) % 2;
        // println!("{}", condition_val);
        condition_permutation[index] = condition_mapping[condition_val];
    }
    return condition_permutation;
}

fn fill_unknowns (conditions: &Vec<char>, mask: usize, unknown_indices: &Vec<usize>) -> Vec<char> {
    let n_indices = unknown_indices.len();
    let condition_mapping = [OPERATIONAL_CHAR, DAMAGED_CHAR];

    // let mut total = 0;
    // for i in 0..unknown_indices.len() {
    let mut condition_permutation = conditions.clone();

    for (idx, index) in unknown_indices.iter().enumerate() {
        // println!("mask: {}, idx: {}, n_indices: {}", mask, idx, n_indices);
        let condition_val = (mask >> idx) % 2;
        // println!("{}", condition_val);
        condition_permutation[*index] = condition_mapping[condition_val];
    }

    return condition_permutation;
}

fn condition_representation (conditions: &Vec<char>) -> Vec<usize> {
    let mut block_representation: Vec<usize> = vec![0];

    for c in conditions {
        if *c == UNKNOWN_CHAR {
            panic!()
        }

        if *c == DAMAGED_CHAR {
            *block_representation.last_mut().unwrap() += 1;
            continue;
        }

        if *block_representation.last().unwrap() == 0 {
            continue;
        }

        block_representation.push(0);
    }
    if *block_representation.last().unwrap() == 0 {
        block_representation.pop();
    }
    return block_representation;
}

fn check_valid (conditions: &Vec<char>, damaged_blocks: &Vec<usize>) -> bool {
    let block_representation = condition_representation(conditions);
    // println!("Representation: {:?}", block_representation);

    return block_representation == *damaged_blocks;
}

fn parse_input () -> Vec<SpringRow> {
    let mut springs = vec![];

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        let sections = line.split_whitespace().collect::<Vec<_>>();

        let spring = SpringRow {
            conditions: sections[0].chars().collect(),
            damaged_blocks: sections[1].split(",").map(|v| v.parse::<usize>().unwrap()).collect()
        };
        springs.push(spring);
    }

    return springs;
}

#[derive(Debug)]
#[derive(Clone)]
struct SpringRow {
    conditions: Vec<char>,
    damaged_blocks: Vec<usize>
}

impl SpringRow {
    fn repeat (&self, n_repeats: usize) -> Self {
        let mut repeated_conditons = vec![];
        let mut repeated_damage = vec![];

        for _ in 0..n_repeats {
            repeated_conditons.append(&mut self.conditions.clone());
            repeated_conditons.push(UNKNOWN_CHAR);

            repeated_damage.append(&mut self.damaged_blocks.clone());
        }
        repeated_conditons.pop();

        return SpringRow {
            conditions: repeated_conditons,
            damaged_blocks: repeated_damage
        }
    }
}