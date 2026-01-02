use std::collections::HashMap;
use std::{collections::HashSet, fs};
use std::cmp::max;

const FILENAME: &str = "./inputs/input.txt";

fn main() {
    let cards = parse_input();
    println!("{} cards", cards.len());

    let mut scores = vec![];

    let mut total = 0;
    for card in cards.iter() {
        let n_winning = card.winning_numbers.iter().filter(|n| card.own_numbers.contains(n)).count();
        scores.push(n_winning);

        if n_winning >= 1 {
             let card_score = 2_u32.pow(n_winning as u32 - 1);
            total += card_score;
        }
    }
    println!("Part 1: {}", total);

    let mut total_cards = HashMap::new();

    let mut total_2 = 0;

    for i in 0..scores.len() {
        let score = scores[i];
        let card_count = *total_cards.get(&i).unwrap_or(&1);
        println!("Card {}, score {}, count {}", i + 1, score, card_count);
        total_2 += card_count;

        for j in 0..score {
            let current_count = *total_cards.get(&(i + j + 1)).unwrap_or(&1);
            total_cards.insert(i + j + 1, card_count + current_count);
        }
    }

    println!("Part 2: {}", total_2)

    // for i in (0..scores.len()).rev() {
    //     let score = scores[i];

    //     let mut card_count = 1;
    //     for j in i..(i+score) {
    //         if total_cards.contains_key(&j) {
    //             card_count += total_cards.get(&j).unwrap();
    //         } else {

    //         }
            
    //     }

    //     total_cards.insert(i, card_count);
    //     println!("Card {}, count {}", i + 1, card_count);
    // }
}

fn parse_input () -> Vec<Card> {
    let mut cards = vec![];

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        let sections = line.split(": ").collect::<Vec<_>>();
        let number_sections = sections[1].split(" | ").collect::<Vec<_>>();

        cards.push(Card {
            winning_numbers: number_sections[0].split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<HashSet<_>>(),
            own_numbers: number_sections[1].split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<HashSet<_>>(),
        });
    }

    return cards;
}

struct Card {
    winning_numbers: HashSet<u32>,
    own_numbers: HashSet<u32>
}