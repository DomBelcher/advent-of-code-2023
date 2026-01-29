use core::fmt;
use std::{collections::HashSet, fmt::{Display, Formatter}, fs, i32};

const FILENAME: &str = "./inputs/input.txt";

fn main() {
    let (
        seeds,
        maps
        // seed_to_soil_map,
        // soil_to_fert_map,
        // fert_to_water_map,
        // water_to_light_map,
        // light_to_temp_map,
        // temp_to_humidity_map,
        // humidity_to_loc_map
    ) = parse_input();

    let mut lowest_location = i64::MAX;

    for seed in seeds {
        // // println!("-----");
        // // println!("seed: {}", seed);
        // let soil = map_by(seed, &maps[0]);
        // // println!("soil: {}", soil);
        // let fert = map_by(soil, &maps[1]);
        // // println!("fert: {}", fert);
        // let water = map_by(fert, &maps[2]);
        // // println!("water: {}", water);
        // let light = map_by(water, &maps[3]);
        // // println!("light: {}", light);
        // let temp = map_by(light, &maps[4]);
        // // println!("temp: {}", temp);
        // let humidity = map_by(temp, &maps[5]);
        // // println!("humidity: {}", humidity);
        // let loc = map_by(humidity, &maps[6]);
        // // println!("loc: {}", loc);
        let loc = apply_mapping(seed, &maps);

        if loc < lowest_location {
            lowest_location = loc;
        }
    }

    println!("Smallest location: {}", lowest_location);
}

fn apply_mapping (seed_value: i64, maps: &[HashSet<RangeMap>; 7]) -> i64 {
    let mut value = seed_value;
    for mapping in maps {
        value = map_by(value, mapping);
    }

    return value
}

fn map_range (range: (i64, i64), maps: &HashSet<RangeMap>) -> HashSet<(i64, i64)> {
    let mut mapped_ranges = HashSet::new();

    for map in maps {
        // mapping is wholly within range
        if range.0 <= map.source_bounds.0 && map.source_bounds.1 <= range.1 {

        }

        // range is wholly within mapping
        if map.source_bounds.0 <= range.0 && range.1 <= map.source_bounds.1 {
            mapped_ranges.insert((
                map.shift(range.0),
                map.shift(range.1)
            ));
            continue;
        }

        // no overlap
        if map.source_bounds.1 < range.0 || range.1 < map.source_bounds.0 {
            mapped_ranges.insert(range);
            continue;
        }

        // range and mapping intersect

    }

    return mapped_ranges;
}

fn map_by (value: i64, maps: &HashSet<RangeMap>) -> i64 {
    for map in maps {
        if map.source_contains(value) {
            return map.map(value).unwrap();
        }
    }

    return value
}

fn parse_input () -> (
    Vec<i64>,
    [HashSet<RangeMap>; 7]
    // HashSet<RangeMap>,
    // HashSet<RangeMap>,
    // HashSet<RangeMap>,
    // HashSet<RangeMap>,
    // HashSet<RangeMap>,
    // HashSet<RangeMap>,
    // HashSet<RangeMap>
) {
    let mut seeds = vec![];

    // let mut input_mode = InputMode::UNKNOWN;
    let mut input_mode = "";

    let mut seed_to_soil_map = HashSet::new();
    let mut soil_to_fert_map = HashSet::new();
    let mut fert_to_water_map = HashSet::new();
    let mut water_to_light_map = HashSet::new();
    let mut light_to_temp_map = HashSet::new();
    let mut temp_to_humidity_map = HashSet::new();
    let mut humidity_to_loc_map = HashSet::new();

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        if line.starts_with("seeds") {
            // input_mode = InputMode::SEEDS;
            input_mode = "seeds";
            let sections = line.split("seeds:").collect::<Vec<_>>();
            seeds = sections[1].split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();
            continue;
        }

        if line.contains("map:") {
            input_mode = line.split_whitespace().next().unwrap();
            continue;
        }

        if line.len() == 0 {
            input_mode = "";
            continue;
        }

        if input_mode == "seed-to-soil" {
            let new_map = RangeMap::from_input(line);
            seed_to_soil_map.insert(new_map);
        } else if input_mode == "soil-to-fertilizer" {
            soil_to_fert_map.insert(RangeMap::from_input(line));
        } else if input_mode == "fertilizer-to-water" {
            fert_to_water_map.insert(RangeMap::from_input(line));
        } else if input_mode == "water-to-light" {
            water_to_light_map.insert(RangeMap::from_input(line));
        } else if input_mode == "light-to-temperature" {
            light_to_temp_map.insert(RangeMap::from_input(line));
        } else if input_mode == "temperature-to-humidity" {
            temp_to_humidity_map.insert(RangeMap::from_input(line));
        } else if input_mode == "humidity-to-location" {
            humidity_to_loc_map.insert(RangeMap::from_input(line));
        } else {
            panic!();
        }

    }

    
    return (
        seeds,
        [seed_to_soil_map, soil_to_fert_map, fert_to_water_map, water_to_light_map, light_to_temp_map, temp_to_humidity_map, humidity_to_loc_map]
        // seed_to_soil_map,
        // soil_to_fert_map,
        // fert_to_water_map,
        // water_to_light_map,
        // light_to_temp_map,
        // temp_to_humidity_map,
        // humidity_to_loc_map
    )
}

#[derive(PartialEq, Eq, Debug, Hash)]
struct RangeMap {
    source_bounds: (i64, i64),
    dest_bounds: (i64, i64)
}

impl RangeMap {
    fn from_input (input: &str) -> RangeMap {
        let sections = input.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();

        return RangeMap {
            source_bounds: (sections[1], sections[1] + sections[2]),
            dest_bounds: (sections[0], sections[0] + sections[2])
        }
    }

    fn source_contains (&self, value: i64) -> bool {
        return self.source_bounds.0 <= value && value < self.source_bounds.1
    }

    fn dest_contains (&self, value: i64) -> bool {
        return self.dest_bounds.0 <= value && value < self.dest_bounds.1
    }

    fn map (&self, value: i64) -> Option<i64> {
        if self.source_contains(value) {
            return Some(self.shift(value))
        }
        return None
    }

    fn shift (&self, value: i64) -> i64 {
        return value - self.source_bounds.0 + self.dest_bounds.0;
    }
}

// #[derive(PartialEq, Eq, Debug)]
// enum InputMode {
//     UNKNOWN,
//     SEEDS,
//     SEED_TO_SOIL,
//     SOIL_TO_FERTILISER,
//     FERTILISER_TO_WATER,
//     WATER_TO_LIGHT,
//     LIGHT_TO_TEMP,
//     TEMP_TO_HUMIDITY,
//     HUMIDITY_TO_LOC
// }

// use InputMode::*;

// impl InputMode {
//     fn prefix (self) -> &'static str {
//         match self {
//             UNKNOWN => "",
//             SEEDS => "",
//             SEED_TO_SOIL => "seed-to-soil",
//             SOIL_TO_FERTILISER => "soil-to-fertilizer",
//             FERTILISER_TO_WATER => "fertilizer-to-water",
//             WATER_TO_LIGHT => "water-to-light",
//             LIGHT_TO_TEMP => "seed-to-soil",
//             TEMP_TO_HUMIDITY => "seed-to-soil",
//             HUMIDITY_TO_LOC => "seed-to-soil",
//         }
//     }
// }

// impl Display for InputMode {
//     fn fmt (&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
//         return match self {
//             Self::UNKNOWN => write!(fmt, "UNKNOWN"),
//             Self::SEEDS => write!(fmt, "SEEDS"),
//             Self::SEED_TO_SOIL => write!(fmt, "SEED_TO_SOIL"),
//             Self::SOIL_TO_FERTILISER => write!(fmt, "SOIL_TO_FERTILISER"),
//             Self::FERTILISER_TO_WATER => write!(fmt, "FERTILISER_TO_WATER"),
//             Self::WATER_TO_LIGHT => write!(fmt, "WATER_TO_LIGHT"),
//             Self::LIGHT_TO_TEMP => write!(fmt, "LIGHT_TO_TEMP"),
//             Self::TEMP_TO_HUMIDITY => write!(fmt, "TEMP_TO_HUMIDITY"),
//             Self::HUMIDITY_TO_LOC => write!(fmt, "HUMIDITY_TO_LOC"),
//         }
//     }
// }