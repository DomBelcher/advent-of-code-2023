use std::{collections::HashSet, fs};
use std::cmp::{min, max};

const FILENAME: &str = "./inputs/input.txt";

const GALAXY_CHAR: char = '#';
const EMPTY_CHAR: char = '.';

const EXPANSION_FACTOR: usize = 1_000_000;

fn main() {
    let galaxy_locs = parse_input();

    let mut total_dist = 0;

    for loc1_idx in 0..galaxy_locs.len() {
        let loc1 = galaxy_locs[loc1_idx];

        for loc2_idx in (loc1_idx+1)..galaxy_locs.len() {
            let loc2 = galaxy_locs[loc2_idx];
            let dist = manhattan(loc1, loc2);
            total_dist += dist;
        }
    }

    println!("Distance: {}", total_dist);
}

fn manhattan (loc1: (usize, usize), loc2: (usize, usize)) -> usize {
    let min_x = min(loc1.0, loc2.0) as i32;
    let max_x = max(loc1.0, loc2.0) as i32;
    let min_y = min(loc1.1, loc2.1) as i32;
    let max_y = max(loc1.1, loc2.1) as i32;

    return (max_x - min_x + max_y - min_y) as usize
}

fn parse_input () -> Vec<(usize, usize)> {
    let mut galaxy_locations = vec![];
    let mut empty_cols = HashSet::new();

    let mut expanded_rows: usize = 0;

    for (row_idx, line) in fs::read_to_string(FILENAME).unwrap().lines().enumerate() {
        if row_idx == 0 {
            empty_cols = (0..line.len()).collect::<HashSet<_>>();
        }

        if line.chars().all(|c| c == EMPTY_CHAR) {
            expanded_rows += 1;
            continue;
        }

        for (col_idx, c) in line.chars().enumerate() {
            if c == GALAXY_CHAR {
                galaxy_locations.push((row_idx + expanded_rows * (EXPANSION_FACTOR - 1), col_idx));
                empty_cols.remove(&col_idx);
            }
        }
    }

    let mut ordered_empty_cols = empty_cols.iter().collect::<Vec<_>>();
    ordered_empty_cols.sort();
    galaxy_locations.sort_by(|(_, col_a), (_, col_b)| col_a.cmp(col_b));

    let mut expanded_galaxy_locations = vec![];

    let mut expanded_cols: usize = 0;
    for loc in galaxy_locations.iter() {
        loop {
            if expanded_cols < ordered_empty_cols.len() && loc.1 > *ordered_empty_cols[expanded_cols] {
                expanded_cols += 1;
            } else {
                break;
            }
        }
        expanded_galaxy_locations.push((loc.0, loc.1 + expanded_cols * (EXPANSION_FACTOR - 1) ));
        
    }

    return expanded_galaxy_locations;
}