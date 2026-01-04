use std::fs;

const FILENAME: &str = "./inputs/input.txt";

fn main() {
    let sequences = parse_input();
    // println!("{:?}", sequences[0]);
    // println!("{:?}", derivative(&sequences[0]));

    let mut total = 0;
    for seq in sequences.iter() {
        let val = next_val(seq);
        total += val;
    }
    println!("Part 1: {}", total);

    let mut total_2 = 0;
    for seq in sequences.iter() {
        let val = prev_val(seq);
        // println!("Previous: {}", val);
        total_2 += val;
    }
    println!("Part 2: {}", total_2);
}

fn extend_sequence (sequence: &Vec<i32>) -> Vec<i32> {
    let mut new_sequence = sequence.clone();
    new_sequence.push(next_val(sequence));
    return new_sequence;
}

fn prev_val (sequence: &Vec<i32>) -> i32 {
    if sequence.iter().all(|v| *v == 0) {
        return 0;
    }

    return sequence.first().unwrap() - prev_val(&derivative(sequence))
}

fn next_val (sequence: &Vec<i32>) -> i32 {
    if sequence.iter().all(|v| *v == 0) {
        return 0;
    }

    return sequence.last().unwrap() + next_val(&derivative(sequence))
}

fn derivative (sequence: &Vec<i32>) -> Vec<i32> {
    let mut d = vec![];

    let mut prev = sequence[0];

    for val in sequence.iter().skip(1) {
        d.push(val - prev);
        prev = *val;
    }

    return d;
    // return sequence.iter().skip(1).fold(vec![sequence[0]], |mut acc, curr| { acc.push(*curr - acc.last().unwrap()); acc })
}


fn parse_input () -> Vec<Vec<i32>> {
    let mut sequences = vec![];
    
    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        sequences.push(
            line.split_whitespace().map(|v| v.parse::<i32>().unwrap()).collect::<Vec<_>>()
        );
    }

    return sequences;
}