use std::fs;

const FILENAME: &str = "./inputs/input.txt";

const CUBES : (usize, usize, usize) = (12, 13, 14);

const RED: &str = "red";
const GREEN: &str = "green";
const BLUE: &str = "blue";

fn main() {
    let games = parse_input();

    let mut total_1 = 0;
    let mut power_total = 0;

    for (idx, game) in games.iter().enumerate() {
        let mut min_cubes = (0, 0, 0);

        let mut valid = true;
        for cubes in game.cubes.iter() {
            if cubes.0 > CUBES.0 || cubes.1 > CUBES.1 || cubes.2 > CUBES.2 {
                valid = false;
            }

            if cubes.0 > min_cubes.0 {
                min_cubes.0 = cubes.0;
            }
            if cubes.1 > min_cubes.1 {
                min_cubes.1 = cubes.1;
            }
            if cubes.2 > min_cubes.2 {
                min_cubes.2 = cubes.2;
            }
        }

        if valid {
            println!("Game {} is valid", game.id);
            total_1 += game.id;
        }
        power_total += min_cubes.0 * min_cubes.1 * min_cubes.2;
    }

    println!("Part 1: {}", total_1);
    println!("Part 2: {}", power_total);
}

fn parse_input () -> Vec<Game> {
    let mut games = vec![];

    for line in fs::read_to_string(FILENAME).unwrap().lines() {

        let sections = line.split(": ").collect::<Vec<&str>>();
        let game_id = sections[0].split("Game ").collect::<Vec<_>>()[1].parse::<usize>().unwrap();

        let mut game = Game { id: game_id, cubes: vec![] };

        let draws = sections[1].split("; ").collect::<Vec<&str>>();
        for draw in draws {
            let cubes = draw.split(", ").collect::<Vec<&str>>();

            let mut cube_vals = (0, 0, 0);
            for cube in cubes {
                let value = cube.split_whitespace().next().unwrap().parse::<usize>().unwrap();
                if cube.contains(RED) {
                    cube_vals.0 = value
                } else if cube.contains(GREEN) {
                    cube_vals.1 = value
                } else if cube.contains(BLUE) {
                    cube_vals.2 = value
                } 
            }

            game.cubes.push(cube_vals);
        }
        games.push(game);
    }

    return games;
}

struct Game {
    id: usize,
    cubes: Vec<(usize, usize, usize)>
}