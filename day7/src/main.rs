use std::{cmp::Ordering, collections::HashMap, fs};

const FILENAME: &str = "./inputs/input.txt";

const CARDS: [char; 13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
const JOKER_CARDS: [char; 13] = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];


fn main() {
    let hands = parse_input();
    let n_hands = hands.len();

    let mut sorted_hands = hands.clone();
    let mut joker_hands = hands.clone();

    sorted_hands.sort_by(make_sort_fn(&score_hand, &card_val));
    joker_hands.sort_by(make_sort_fn(&joker_score, &joker_card_val));

    // println!("{:?}", sorted_hands);

    let mut total_score = 0;

    for (idx, hand) in sorted_hands.iter().enumerate() {
        // println!("{:?} - {}", hand, classify_hand(hand.0));
        total_score += (n_hands - idx) as u32 * hand.1;
    }

    println!("Part 1: {}", total_score);

    let mut joker_score = 0;
    for (idx, hand) in joker_hands.iter().enumerate() {
        // println!("{:?} - {}", hand, classify_hand(hand.0));
        joker_score += (n_hands - idx) as u32 * hand.1;
    }
    println!("Part 2: {}", joker_score);
}

fn make_sort_fn (score_fn: &impl Fn ([char; 5]) -> u32, value_fn: &impl Fn (char) -> Option<usize>) -> impl Fn (&([char; 5], u32), &([char; 5], u32)) -> Ordering {
    let _sort = |hand_1: &([char; 5], u32), hand_2: &([char; 5], u32)| {
        return sort_fn(hand_1, hand_2, score_fn, value_fn)
    };
    return _sort
}

fn sort_fn (hand_1: &([char; 5], u32), hand_2: &([char; 5], u32), score_fn: &impl Fn ([char; 5]) -> u32, value_fn: &impl Fn (char) -> Option<usize>) -> Ordering {
    let score_1 = score_fn(hand_1.0);
    let score_2 = score_fn(hand_2.0);

    if score_1 != score_2 {
        return score_2.cmp(&score_1)
    }

    for i in 0..5 {
        if hand_1.0[i] != hand_2.0[i] {
            let val_1 = value_fn(hand_1.0[i]).unwrap();
            let val_2 = value_fn(hand_2.0[i]).unwrap();

            return val_1.cmp(&val_2);
        }
    }

    println!("{:?} and {:?} are equal", hand_1, hand_2);
    return Ordering::Equal;
}

fn card_val (card: char) -> Option<usize> {
    return CARDS.iter().position(|c| *c == card)
}

fn joker_card_val (card: char) -> Option<usize> {
    return JOKER_CARDS.iter().position(|c| *c == card)
}

fn parse_input () -> Vec<([char; 5], u32)> {
    let mut hands = vec![];

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        hands.push(parse_hand(line));
    }

    return hands
}

fn parse_hand (input: &str) -> ([char; 5], u32) {
    let sections = input.split_whitespace().collect::<Vec<_>>();
    let hand = sections[0].chars().collect::<Vec<_>>().try_into().unwrap();
    let bid = sections[1].parse::<u32>().unwrap();

    return (hand, bid);
}

fn classify_hand (hand: [char; 5]) -> &'static str {
    let score = score_hand(hand);

    match score {
        7 => "five of a kind",
        6 => "four of a kind",
        5 => "full house",
        4 => "three of a kind",
        3 => "two pair",
        2 => "pair",
        1 => "high card",
        _ => panic!()
    }
}

fn joker_hand (hand: [char; 5]) -> [char; 5] {
    if !hand.contains(&'J') {
        return hand;
    }

    let mut best_score = 0;
    let mut best_hand = hand;
    for card in JOKER_CARDS {
        let joker_hand = hand.map(|c| if c == 'J' { card } else { c } );
        let score_with_joker = score_hand(joker_hand);
        if score_with_joker > best_score {
            best_hand = joker_hand;
            best_score = score_with_joker;
        }
    }

    return best_hand;
}

fn joker_score (hand: [char; 5]) -> u32 { 
    if !hand.contains(&'J') {
        return score_hand(hand);
    }

    let mut best_score = 0;
    // let mut best_hand = hand;
    for card in JOKER_CARDS {
        let joker_hand = hand.map(|c| if c == 'J' { card } else { c } );
        let score_with_joker = score_hand(joker_hand);
        if score_with_joker > best_score {
            // best_hand = joker_hand;
            best_score = score_with_joker;
        }
    }

    return best_score;
}

fn score_hand (hand: [char; 5]) -> u32 {
    // println!("Scoring hand: {:?}", hand);
    let counts = hand_counts(hand);

    // five of a kind
    if counts.iter().next().unwrap().1 == &5 {
        // println!("five of a kind");
        return 7;
    }

    // four of a kind
    if counts.values().any(|v| *v == 4) {
        // println!("four of a kind");
        return 6;
    }

    // full house
    if counts.len() == 2 && counts.values().any(|v| *v == 3) {
        // println!("full house");
        return 5;
    }

    // three of a kind
    if counts.values().any(|v| *v == 3) {
        // println!("three of a kind");
        return 4;
    }

    // two pair
    if counts.len() == 3 && counts.values().any(|v| *v == 2) {
        // println!("two pair");
        return 3;
    }

    // pair
    if counts.values().any(|v| *v == 2) {
        // println!("pair");
        return 2;
    }

    // high card
    // println!("high card");
    return 1;
}

fn hand_counts (hand: [char; 5]) -> HashMap<char, usize> {
    let mut counts = HashMap::new();

    for c in hand {
        if counts.contains_key(&c) {
            let count = counts.get(&c).unwrap();
            counts.insert(c, count + 1);
        } else {
            counts.insert(c, 1);
        }
    }

    return counts;
}