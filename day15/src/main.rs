use std::{collections::HashMap, fs};

const FILENAME: &str = "./inputs/input.txt";

const HASH: [char; 4] = ['H', 'A', 'S', 'H'];


fn main() {
    let ivs = parse_input();
    println!("HASH: {}", hash(&HASH.to_vec()));

    let mut total = 0;
    for iv in ivs.iter() {
        total += hash(iv) as u32;
    }

    println!("Part 1: {}", total);

    let mut boxes: HashMap<u8, Vec<Lens>> = HashMap::new();

    for iv in ivs.iter() {

        if iv.contains(&'-') {
            let lens_id = iv.split(|c| *c == '-').next().unwrap().to_vec();
            let box_id = hash(&lens_id);
            box_remove(&mut boxes, box_id, &lens_id);
        } else if iv.contains(&'=') {
            let sections = iv.split(|c| *c == '=').collect::<Vec<_>>();
            let lens_id = sections[0].to_vec();
            let box_id = hash(&lens_id);
            let lens_power = sections[1].iter().collect::<String>().parse::<u8>().unwrap();
            let lens = Lens {
                id: lens_id,
                power: lens_power
            };
            box_insert(&mut boxes, box_id, lens);
        }
    }

    let mut total_2 = 0;

    for (box_id, lenses) in boxes.iter() {
        for (idx, lens) in lenses.iter().enumerate() {
            total_2 += (*box_id as u32 + 1) * (idx as u32 + 1) * (lens.power as u32)
        }
    }
    println!("Part 2: {}", total_2);
}

fn box_insert (boxes: &mut HashMap<u8, Vec<Lens>>, box_id: u8, lens: Lens) {
    if !boxes.contains_key(&box_id) {
        boxes.insert(box_id, vec![lens]);
        return;
    }

    let b = boxes.get_mut(&box_id).unwrap();

    for i in 0..b.len() {
        if b[i].id == lens.id {
            b[i] = lens;
            return;
        }
    }

    b.push(lens);
}

fn box_remove (boxes: &mut HashMap<u8, Vec<Lens>>, box_id: u8, lens_id: &Vec<char>) {
    if !boxes.contains_key(&box_id) {
        return;
    }

    let b = boxes.get_mut(&box_id).unwrap();

    for i in 0..b.len() {
        if b[i].id == *lens_id {
            b.remove(i);
            return;
        }
    }
}

fn hash (string: &Vec<char>) -> u8 {
    let mut cv = 0;
    for c in string {
        cv = hash_char(cv, *c);
    }
    return cv
}

fn hash_char (cv: u8, c: char) -> u8 {
    let mut curr = cv as u32 + c as u32;
    curr *= 17;
    return (curr % 256) as u8
}

fn parse_input () -> Vec<Vec<char>> {
    let mut ivs = vec![];

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        for iv in line.split(",") {
            ivs.push(iv.chars().collect())
        }
    }

    return ivs
}

#[derive(Clone, Debug)]
struct Lens {
    id: Vec<char>,
    power: u8
}