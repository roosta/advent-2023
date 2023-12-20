use std::fs;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Card {
    label: char,
    value: i64,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    hand: String,
    bid: i64,
    class: String,
}

impl Hand {
    const CARD_RANKS: [char; 13] = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];
    fn class(&self) -> &str {
        &self.class
    }
    fn classify(&mut self) {
        let mut sorted = self.cards.clone();
        sorted.sort_by(|a, b| a.value.cmp(&b.value));
        let iter: Vec<i64> = sorted.iter().map(|c| c.value).collect();
        match &iter[..] {
            [a, b, c, d, e] if a == b && b == c && c == d && d == e => {
                self.class = "Five of a kind".to_string();
            }
            [a, rest @ .., _] if rest.iter().all(|x| x == a) => {
                self.class = "Four of a kind".to_string();
            }
            [_, rest @ .., a] if rest.iter().all(|x| x == a) => {
                self.class = "Four of a kind".to_string();
            }
            [a, b, c, d, e] if a == b && b == c && d == e => {
                self.class = "Full house".to_string();
            }
            [a, b, c, d, e] if a == b && c == d && d == e => {
                self.class = "Full house".to_string();
            }
            [a, b, c, _, _] if a == b && b == c => {
                self.class = "Three of a kind".to_string();
            }
            [_, a, b, c, _] if a == b && b == c => {
                self.class = "Three of a kind".to_string();
            }
            [_, _, a, b, c] if a == b && b == c => {
                self.class = "Three of a kind".to_string();
            }
            [a, b, c, _, _] if a == b && b == c => {
                self.class = "Three of a kind".to_string();
            }
            [a, b, c, d, _] if a == b && c == d => {
                self.class = "Two pairs".to_string();
            }
            [_, a, b, c, d] if a == b && c == d => {
                self.class = "Two pairs".to_string();
            }
            [a, b, _, c, d] if a == b && c == d => {
                self.class = "Two pairs".to_string();
            }
            [a, b, _, _, _] if a == b => {
                self.class = "One pair".to_string();
            }
            [_, a, b, _, _] if a == b => {
                self.class = "One pair".to_string();
            }
            [_, _, a, b, _] if a == b => {
                self.class = "One pair".to_string();
            }
            [_, _, _, a, b] if a == b => {
                self.class = "One pair".to_string();
            }
            [a, b, c, d, e] if a != b && b != c && c != d && d != e => {
                self.class = "Highcard".to_string();
            }
            _ => self.class = "No hand".to_string(),
        }
    }
}

fn parse_card(label: char) -> Card {
    return Card {
        label,
        value: Hand::CARD_RANKS.iter().position(|c| c == &label).unwrap() as i64 + 1,
    };
}

fn parse_hand(line: &str) -> Hand {
    let v: Vec<&str> = line.split(' ').collect();
    let [cards, bid] = &v[..] else {
        panic!("Problem parsing line \"{}\"", line)
    };
    return Hand {
        cards: cards.chars().map(parse_card).collect(),
        hand: cards.to_string(),
        bid: bid.parse().unwrap(),
        class: "Unknown".to_string(),
    };
}

fn main() {
    let input = fs::read_to_string("input/day7.txt").unwrap();
    let mut hands: Vec<Hand> = input.lines().map(parse_hand).collect();
    for hand in &mut hands {
        hand.classify();
    }
    let hand_ranks = HashMap::from([
        ("Five of a kind",  1),
        ("Four of a kind",  2),
        ("Full house",      3),
        ("Three of a kind", 4),
        ("Two pairs",       5),
        ("One pair",        6),
        ("Highcard",        7),
        ("No hand",         8),

    ]);
    hands.sort_by(|a, b| {
        let ra = hand_ranks.get(a.class()).unwrap();
        let rb = hand_ranks.get(b.class()).unwrap();
        if ra == rb {
            for (card_a, card_b) in a.cards.iter().zip(b.cards.iter()) {
                if card_a.value == card_b.value {
                    continue;
                } else {
                    return card_a.value.cmp(&card_b.value)
                }
            }
        }
        ra.cmp(rb)

    });
    let mut result = 0;
    for (i, hand) in hands.iter().enumerate() {
        let rank = (hands.len() - i) as i64;
        let bid = hand.bid;
        result += rank * bid;
        // println!("hand: {}, rank {}, bid: {}", hand.hand, rank, bid);
    }
    println!("result = {}", result);
}
