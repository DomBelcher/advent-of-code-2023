use std::{collections::{HashMap, HashSet}, fs};

const FILENAME: &str = "./inputs/input.txt";
const N_CYCLES: usize = 1000000000;

const ROCK_CHAR: char = 'O';
const CUBE_CHAR: char = '#';
const EMPTY_CHAR: char = '.';

const DIRS: Dirs = Dirs {
    NORTH: (0, -1),
    EAST: (1, 0),
    SOUTH: (0, 1),
    WEST: (-1, 0)
};

fn main() {
    let mut grid = parse_input();
    tilt_north(&mut grid);

    let height = grid.len();

    let mut total = 0;
    for (idx, row) in grid.iter().enumerate() {
        total += (height - idx) * row.iter().filter(|c| **c == ROCK_CHAR).count();
    }
    
    println!("Part 1: {}", total);

    let mut grid = parse_input();
    let mut last_grid = grid.clone();

    let mut cache: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    let mut fixed_points: HashMap<Vec<Vec<char>>, usize> = HashMap::new();

    let mut i = 0;
    loop {
        println!("{}", i);
        if i >= N_CYCLES {
            break;
        }
        if cache.contains_key(&grid) {
            let cycle_count;
            if fixed_points.contains_key(&grid) {
                cycle_count = *fixed_points.get(&grid).unwrap();
            } else {
                cycle_count = i - cache.get(&grid).unwrap();
                fixed_points.insert(grid.clone(), cycle_count);
            }
            println!("Fixed point with {} cycles", cycle_count);
            if cycle_count + i <= N_CYCLES {
                i += ((N_CYCLES - i) / cycle_count) * cycle_count;
                continue;
            }
        }

        spin_cycle(&mut grid);

        cache.insert(last_grid, i);
        last_grid = grid.clone();
        i += 1;

    }

    let mut total_2 = 0;
    for (idx, row) in grid.iter().enumerate() {
        total_2 += (height - idx) * row.iter().filter(|c| **c == ROCK_CHAR).count();
    }
    
    println!("Part 2: {}", total_2);
}

fn spin_cycle (grid: &mut Vec<Vec<char>>) {
    tilt_north(grid);
    tilt_west(grid);
    tilt_south(grid);
    tilt_east(grid);
}

fn tilt_north (grid: &mut Vec<Vec<char>>) {
    let width = grid[0].len();
    let height = grid.len();

    for x in 0..width {
        for y in 0..height {
            if grid[y][x] == ROCK_CHAR {
                roll_rock(grid, (x, y), DIRS.NORTH);
            }
        }
    }
}

fn tilt_east (grid: &mut Vec<Vec<char>>) {
    let width = grid[0].len();
    let height = grid.len();

    for y in 0..height {
        for x in (0..width).rev() {
            if grid[y][x] == ROCK_CHAR {
                roll_rock(grid, (x, y), DIRS.EAST);
            }
        }
    }
}

fn tilt_south (grid: &mut Vec<Vec<char>>) {
    let width = grid[0].len();
    let height = grid.len();

    for x in 0..width {
        for y in (0..height).rev() {
            if grid[y][x] == ROCK_CHAR {
                roll_rock(grid, (x, y), DIRS.SOUTH);
            }
        }
    }
}

fn tilt_west (grid: &mut Vec<Vec<char>>) {
    let width = grid[0].len();
    let height = grid.len();

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == ROCK_CHAR {
                roll_rock(grid, (x, y), DIRS.WEST);
            }
        }
    }
}

fn roll_rock (grid: &mut Vec<Vec<char>>, coords: (usize, usize), dir: (i8, i8)) {
    let width = grid[0].len();
    let height = grid.len();

    let mut src_coords = coords;
    loop {
        let dc = (src_coords.0 as i8 + dir.0, src_coords.1 as i8 + dir.1);

        if dc.0 < 0 || dc.0 >= width as i8 || dc.1 < 0 || dc.1 >= height as i8 {
            break;
        }
        let dest_coords = (dc.0 as usize, dc.1 as usize);
        if grid[dest_coords.1][dest_coords.0] != EMPTY_CHAR {
            break;
        }
        swap(grid, src_coords, dest_coords);

        src_coords = dest_coords;
    }
}

fn roll_north (grid: &mut Vec<Vec<char>>, coords: (usize, usize)) {
    let mut src_coords = coords;

    loop {
        if src_coords.1 == 0 {
            break;
        }

        let dest_coords = (src_coords.0, src_coords.1 - 1);
        if grid[dest_coords.1][dest_coords.0] != EMPTY_CHAR {
            break;
        }
        swap(grid, src_coords, dest_coords);

        src_coords = dest_coords;
    }
}

fn swap (grid: &mut Vec<Vec<char>>, src_coords: (usize, usize), dest_coords: (usize, usize)) {
    grid[src_coords.1][src_coords.0] = EMPTY_CHAR;
    grid[dest_coords.1][dest_coords.0] = ROCK_CHAR;
}

fn parse_input () -> Vec<Vec<char>> {
    let mut grid = vec![];

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        grid.push(line.chars().collect());
    }

    return grid;
}

struct Dirs {
    NORTH: (i8, i8),
    EAST: (i8, i8),
    SOUTH: (i8, i8),
    WEST: (i8, i8)
}