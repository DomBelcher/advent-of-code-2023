use std::fs;

const FILENAME: &str = "./inputs/input.txt";

fn main() {
    let (races, main_race) = parse_input();
    println!("Races: {}", races.len());

    let mut total = 1;
    for race in races.iter() {
        let mut win_count = 0;
        for t in 0..race.time_allowed {
            let travel_time = race.time_allowed - t;
            let distance = travel_time * t;
            if distance > race.best_distance {
                win_count += 1;
            }
        }
        total *= win_count;
    }

    println!("Part 1: {}", total);

    let mut win_count = 0;
    for t in 0..main_race.time_allowed {
        let travel_time = main_race.time_allowed - t;
        let distance = travel_time * t;
        if distance > main_race.best_distance {
            win_count += 1;
        }
    }
    println!("Part 2: {}", win_count);
}

fn parse_input () -> (Vec<Race>, Race) {
    let mut races = vec![];

    let mut distances = vec![];
    let mut times = vec![];

    let mut overall_time = 0;
    let mut overall_dist = 0;

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        if line.starts_with("Time:") {
            let sections = line.split(":").collect::<Vec<_>>();
            times = sections[1].split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();

            let time_str = sections[1].split_whitespace().collect::<Vec<_>>().join("");
            println!("{}", time_str);
            overall_time = time_str.parse::<u64>().unwrap();
        } else if line.starts_with("Distance:") {
            let sections = line.split(":").collect::<Vec<_>>();
            distances = sections[1].split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();

            let dist_str = sections[1].split_whitespace().collect::<Vec<_>>().join("");
            println!("{}", dist_str);
            overall_dist = dist_str.parse::<u64>().unwrap();
        }
    }

    if distances.len() != times.len() {
        panic!();
    }

    for i in 0..distances.len() {
        races.push(Race {
            time_allowed: times[i],
            best_distance: distances[i]
        })
    }

    return (races, Race {
        time_allowed: overall_time,
        best_distance: overall_dist
    } );
}

struct Race {
    time_allowed: u64,
    best_distance: u64
}