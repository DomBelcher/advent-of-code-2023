use std::{collections::{HashMap, HashSet}, fs};

const FILENAME: &str = "./inputs/input.txt";

const EMPTY_CHAR: char = '.';
const GEAR_CHAR: char = '*';
const DIGIT_CHARS: [char; 10] = ['0','1','2','3','4','5','6','7','8','9'];
const DIRECTIONS: [(i32, i32); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1)
];

fn main() {
    let (digit_map, symbol_coords, gear_coords) = parse_input();

    let mut part_number_positions = HashSet::new();

    for coords in symbol_coords {
        for dir in DIRECTIONS {
            let adjacent_coords = (coords.0 + dir.0, coords.1 + dir.1);
            if !digit_map.contains_key(&adjacent_coords) {
                continue;
            }
            part_number_positions.insert(find_start_positon(adjacent_coords, &digit_map));
        }
    }

    let mut total = 0;
    for start_position in part_number_positions {
        println!("part number starts at: {:?}", start_position);
        let part_no = calc_part_number(start_position, &digit_map);
        println!("Part number: {}", part_no);
        total += part_no;
    }

    println!("Part 1 total: {}", total);

    let mut total_2 = 0;
    for coords in gear_coords {
        let mut adjacent_parts = HashMap::new();
        let mut total_adjacent_parts = 0;
        
        for dir in DIRECTIONS {
            let adjacent_coords = (coords.0 + dir.0, coords.1 + dir.1);
            if !digit_map.contains_key(&adjacent_coords) {
                continue;
            }
            let start_position = find_start_positon(adjacent_coords, &digit_map);

            if adjacent_parts.contains_key(&start_position) {
                continue;
            }
            let part_number = calc_part_number(start_position, &digit_map);
            total_adjacent_parts += 1;
            adjacent_parts.insert(start_position, part_number);
        }

        if total_adjacent_parts == 2 {
            let gear_ratio = adjacent_parts.values().fold(1, |acc, &val| acc * val);
            println!("Gear ratio: {}", gear_ratio);
            total_2 += gear_ratio
        }
    }

    println!("Part 2 total: {}", total_2);
}

fn calc_part_number (start_pos: (i32, i32), digit_map: &HashMap<(i32, i32), u32>) -> u32 {
    let mut part_number = *digit_map.get(&start_pos).unwrap();
    let mut coords = start_pos;

    let mut idx = 0;
    loop {
        let next_coords = (coords.0, coords.1 + 1);
        if digit_map.contains_key(&next_coords) {
            part_number *= 10;
            part_number += digit_map.get(&next_coords).unwrap();
            coords = next_coords;

        } else {
            break
        }
    }

    return part_number;
}

fn find_start_positon (coords: (i32, i32), digit_map: &HashMap<(i32, i32), u32>) -> (i32, i32) {
    let mut start = coords;

    loop {
        let next_coords = (start.0, start.1 - 1);
        if digit_map.contains_key(&next_coords) {
            start = next_coords;
        } else {
            break
        }
    }

    return start;
}

fn parse_input () -> (HashMap<(i32, i32), u32>, HashSet<(i32, i32)>, HashSet<(i32, i32)>) {
    let mut digit_map = HashMap::new();
    let mut symbol_coords = HashSet::new();
    let mut gear_coords = HashSet::new();

    for (row_idx, line) in fs::read_to_string(FILENAME).unwrap().lines().enumerate() {
        for (col_idx, c) in line.chars().enumerate() {
            let coords = (row_idx as i32, col_idx as i32);

            if c.is_digit(10_u32) {
                digit_map.insert(coords, c.to_digit(10_u32).unwrap());
            } else if c == GEAR_CHAR {
                gear_coords.insert(coords);
                symbol_coords.insert(coords);
            } else if c != EMPTY_CHAR {
                symbol_coords.insert(coords);
            }
        }
    }

    return (
        digit_map,
        symbol_coords,
        gear_coords
    )
}