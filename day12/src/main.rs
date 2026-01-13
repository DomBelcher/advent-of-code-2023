use std::{collections::HashMap, fs};

const FILENAME: &str = "./inputs/input.txt";

const OPERATIONAL_CHAR: char = '.';
const DAMAGED_CHAR: char = '#';
const UNKNOWN_CHAR: char = '?';

fn main() {

    let spring_rows = parse_input();
    println!("Rows: {}", spring_rows.len());

    let mut total = 0;
    let mut unfolded_total = 0;

    for (_, row) in spring_rows.iter().enumerate() {
        let valid_permutations = solve(&row.conditions, &row.damaged_blocks);
        total += valid_permutations;

        let unfolded_row = row.repeat(5);
        let count = solve(&unfolded_row.conditions, &unfolded_row.damaged_blocks);
        
        unfolded_total += count;
    }

    println!("Part 1: {}", total);
    println!("Part 2: {}", unfolded_total);

}

fn solve (conditions: &Vec<char>, damaged_blocks: &Vec<usize>) -> usize {
    return _solve (conditions, damaged_blocks, &mut HashMap::new());
}

fn _solve (conditions: &Vec<char>, damaged_blocks: &Vec<usize>, memo: &mut HashMap<(Vec<char>, Vec<usize>), usize>) -> usize {
    if memo.contains_key(&(conditions.clone(), damaged_blocks.clone())) {
        return *memo.get(&(conditions.clone(), damaged_blocks.clone())).unwrap();
    }

    if conditions.len() == 0 && damaged_blocks.len() != 0 {
        return 0;
    }

    if conditions.len() == 0 {
        return 1
    }

    if damaged_blocks.len() == 0 {
        if conditions.iter().any(|c| *c == DAMAGED_CHAR) {
            return 0;
        }
        return 1
    }
    let damaged_total = damaged_blocks.iter().sum::<usize>() + damaged_blocks.len() - 1;

    let mut total = 0;
    let mut idx = 0;
    loop {
        if idx + damaged_total > conditions.len() {
            break
        }
        let consumable = consume(conditions, idx, damaged_blocks[0], damaged_blocks.len() == 1);
        if consumable {
            let next_pointer;
            if damaged_blocks.len() == 1 {
                next_pointer = idx+damaged_blocks[0];
            } else {
                next_pointer = idx+damaged_blocks[0] + 1;
            }

            total += _solve (
                &conditions[next_pointer..].to_vec(),
                &damaged_blocks[1..].to_vec(),
                memo
            );
        }
        if conditions[idx] == DAMAGED_CHAR {
            break
        }
        idx += 1;
    }

    memo.insert((conditions.clone(), damaged_blocks.clone()), total);
    return total;
}

fn consume (conditions: &Vec<char>, index: usize, block_size: usize, last: bool) -> bool {
    if conditions[index..(index+block_size)].iter().any(|c| *c == OPERATIONAL_CHAR) {
        return false
    }
    if !last && conditions[index+block_size] == DAMAGED_CHAR {
        return false
    }

    return true
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
