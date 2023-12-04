use std::fs;

#[derive(Debug)]
struct Card {
    id: i32,
    numbers: Vec<i32>,
    result: Vec<i32>,
}

// Parse number to i32
fn parse_n(s: &str) -> i32 {
    return s.parse().unwrap();
}

// Takes a string of space separated numbers, returns a vector of parsed i32
fn extract_n(s: &str) -> Vec<i32> {
    return s
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(parse_n)
        .collect();
}

fn collect_points(card: Card) -> i32 {
    let mut result = 0;
    for n in &card.numbers {
        if card.result.contains(&n) {
            if result == 0 {
                result = 1;
            } else {
                result *= 2;
            }
        }
    }
    result
}

// Parse an input line, returning  Card struct with parsed data
fn parse_card(line: &str) -> Card {
    let v: Vec<&str> = line.split(':').collect();
    if let [name, rest] = &v[..] {
        let v: Vec<&str> = rest.split('|').collect();
        let sid: &str = name.split(' ').last().unwrap();
        if let [result, numbers] = &v[..] {
            return Card {
                id: parse_n(sid),
                numbers: extract_n(numbers),
                result: extract_n(result)
            }
        }

    }
    panic!("Problem parsing line \"{}\"", line);
}

fn main() {
    let input = fs::read_to_string("input/day4.txt").unwrap();
    let cards = input.lines().map(parse_card);
    let result = cards
        .map(collect_points)
        .reduce(|acc, n| acc + n)
        .unwrap();
    println!("result = {}", result)

}
