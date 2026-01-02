use std::{collections::HashMap, fs};

const FILENAME: &str = "./inputs/input.txt";

const DIGIT_WORDS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn main() {
    let input = parse_input();

    let mut total_1 = 0;

    for line in input.iter() {
        let digits = line
            .iter()
            .filter(|c| c.is_numeric())
            .map(|c| c.to_digit(10_u32).unwrap())
            .collect::<Vec<_>>();
        
        let value = digits.first().unwrap() * 10 + digits.last().unwrap();
        total_1 += value;
    }
    
    println!("Part 1: {}", total_1);
    let base_trie = build_trie();

    let mut total_2 = 0;
    for line in input.iter() {

        let mut tries = vec![];
        tries.push(&base_trie);

        let mut digits = vec![];

        for c in line {
            if c.is_numeric() {
                digits.push(c.to_digit(10_u32).unwrap());

                tries = vec![&base_trie; 1];
                continue;
            }

            let mut new_tries = vec![&base_trie; 1];

            for trie in tries.iter() {
                if trie.leaves.contains_key(c) {
                    if trie.leaves.get(c).unwrap().value.is_some() {
                        digits.push(trie.leaves.get(c).unwrap().value.unwrap());
                        // println!("Parsed digit: {}", trie.leaves.get(c).unwrap().value.unwrap());
                    } else {
                        new_tries.push(trie.leaves.get(c).unwrap());
                    }
                }
            }

            tries = new_tries;
        }
        let value = digits.first().unwrap() * 10 + digits.last().unwrap();
        total_2 += value;
    }
    println!("Part 2: {}", total_2);
}

#[derive(Debug)]
struct TrieNode {
    c: char,
    leaves: Box<HashMap<char, TrieNode>>,
    value: Option<u32>
}

fn build_trie () -> TrieNode {
    let mut base_node = TrieNode {
        c: '_',
        leaves: Box::new(HashMap::new()),
        value: None
    };

    for digit in DIGIT_WORDS {
        let mut parent_node = &mut base_node;

        for (idx, c) in digit.0.chars().enumerate() {
            if idx == 0 && parent_node.leaves.contains_key(&c) {
                parent_node = base_node.leaves.get_mut(&c).unwrap();
                continue;
            }

            let node = TrieNode {
                c,
                leaves: Box::new(HashMap::new()),
                value: if idx == digit.0.len() - 1 { Some(digit.1) } else { None }
            };

            parent_node.leaves.insert(c, node);
            parent_node = parent_node.leaves.get_mut(&c).unwrap();
        }
        
    }

    return base_node;
}

fn parse_input () -> Vec<Vec<char>> {
    let mut input = vec![];

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        input.push(line.chars().collect::<Vec<_>>());
    }

    return input;
}
