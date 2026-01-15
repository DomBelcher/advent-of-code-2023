use std::{collections::HashSet, fs};

const FILENAME: &str = "./inputs/input.txt";

const ASH_CHAR: char = '.';
const ROCKS_CHAR: char = '#';

fn main() {
    let patterns = parse_input();
    println!("Patterns: {}", patterns.len());

    let mut total = 0;

    let mut total_2 = 0;

    for pattern in patterns.iter() {
        let mut pattern_value = 0;
        let mut pattern_value_2 = 0;
        println!("Pattern dimensions: ({},{})", pattern.height, pattern.width);

        for col_idx in 1..pattern.width {
            if pattern.is_mirror_col(col_idx) {
                pattern_value += col_idx;
                println!("Found line of reflection at column {}", col_idx);
            }

            if pattern.mirror_col_exceptions(col_idx) == 1 {
                pattern_value_2 += col_idx;
            }
        }

        for row_idx in 1..pattern.height {
            if pattern.is_mirror_row(row_idx) {
                pattern_value += row_idx * 100;
                println!("Found line of reflection at row {}", row_idx);
            }

            if pattern.mirror_row_exceptions(row_idx) == 1 {
                pattern_value_2 += row_idx * 100;
            }
        }

        println!("Pattern value: {}", pattern_value);
        total += pattern_value;
        total_2 += pattern_value_2
    }

    println!("Part 1: {}", total);
    println!("Part 2: {}", total_2);
}

fn parse_input () -> Vec<Pattern> {
    let mut patterns = vec![];

    let mut row_idx = 0;
    let mut width = 0;
    let mut pattern = Pattern::new();

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        if line.len() == 0 {
            pattern.width = width;
            pattern.height = row_idx;

            patterns.push(pattern);
            pattern = Pattern::new();
            row_idx = 0;
            continue;
        }
        width = line.len() as i32;

        for (col_idx, c) in line.chars().enumerate() {
            let coords = (row_idx, col_idx as i32);

            match c {
                ASH_CHAR => pattern.ash.insert(coords),
                ROCKS_CHAR => pattern.rocks.insert(coords),
                _ => panic!()
            };
        }

        row_idx += 1;
    }
    pattern.width = width;
    pattern.height = row_idx;
    patterns.push(pattern);

    return patterns;
}

struct Pattern {
    rocks: HashSet<(i32, i32)>,
    ash: HashSet<(i32, i32)>,
    width: i32,
    height: i32
}

impl Pattern {
    fn new () -> Pattern {
        return Pattern {
            rocks: HashSet::new(),
            ash: HashSet::new(),
            width: 0,
            height: 0
        };
    }

    fn is_mirror_row (&self, row_idx: i32) -> bool {
        for rock in self.rocks.iter() {
            let other_row = 2 * row_idx - 1 - rock.0;
            if 0 > other_row || other_row >= self.height {
                continue;
            }

            let other_coords = (other_row, rock.1);

            // println!("Mirroring at row: {}", row_idx);
            // println!("Rock: ({},{}) | mirrored: ({},{})", rock.0, rock.1, other_coords.0, other_coords.1);
            // println!("Rock: {} | ash: {}", self.rocks.contains(&other_coords), self.ash.contains(&other_coords));
            if self.ash.contains(&other_coords) {
                return false
            }
        }

        return true
    }

    fn is_mirror_col (&self, col_idx: i32) -> bool {
        for rock in self.rocks.iter() {
            let other_col = 2 * col_idx - 1 - rock.1;
            if 0 > other_col || other_col >= self.width {
                continue;
            }

            let other_coords = (rock.0, other_col);
            if self.ash.contains(&other_coords) {
                return false
            }
        }

        return true
    }

    fn mirror_row_exceptions (&self, row_idx: i32) -> usize {
        let mut exceptions = 0;
        for rock in self.rocks.iter() {
            let other_row = 2 * row_idx - 1 - rock.0;
            if 0 > other_row || other_row >= self.height {
                continue;
            }

            let other_coords = (other_row, rock.1);

            if self.ash.contains(&other_coords) {
                exceptions += 1
            }
        }
        return exceptions
    }

    fn mirror_col_exceptions (&self, col_idx: i32) -> usize {
        let mut exceptions = 0;
        for rock in self.rocks.iter() {
            let other_col = 2 * col_idx - 1 - rock.1;
            if 0 > other_col || other_col >= self.width {
                continue;
            }

            let other_coords = (rock.0, other_col);

            if self.ash.contains(&other_coords) {
                exceptions += 1
            }
        }
        return exceptions
    }
}

// 5
// 0 1 2 3 4 5 6 7 8 9
//            |
// _ _ 9 8 7 6|5 4 3 2 
// 11 - x

// 2
// 0 1 2 3 4 5 6 7 8 9
//      |      
// 5 4 3 2 1 0 _ _ _ _
// 5 - x

