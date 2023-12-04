use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct Card {
    id: i32,
    wins: i32,
    proc: bool,
}

// Parse number to i32
fn parse_n(s: &str) -> i32 {
    return s.parse().unwrap();
}

// Debug function, prints stack sorted, or unsorted
// Useful when checking each iteration of the stack
fn _print_stack(stack: &Vec<Card>, sort: bool) {
    if sort {
        let mut ids: Vec<i32> = stack.iter().map(|x| x.id).collect();
        ids.sort();
        for id in ids {
            print!("[{}] ", id);
        }
    } else {
        for s in stack {
            print!("[{}] ", s.id);
        }
    }
    println!("\n--------------------------------------------");
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

// Sum up wins for a vector of numbers
fn collect_wins(numbers: &Vec<i32>, result: &Vec<i32>) -> i32 {
    let mut wins = 0;
    for n in numbers {
        if result.contains(&n) {
            wins += 1;
        }
    }
    wins
}
// Parse an input line, returning  Card struct with parsed data
fn parse_card(line: &str) -> Card {
    let v: Vec<&str> = line.split(':').collect();
    if let [name, rest] = &v[..] {
        let v: Vec<&str> = rest.split('|').collect();
        let sid: &str = name.split(' ').last().unwrap();
        if let [result, numbers] = &v[..] {
            let n = extract_n(numbers);
            let r = extract_n(result);
            return Card {
                wins: collect_wins(&n, &r),
                id: parse_n(sid),
                proc: false,
            }
        }

    }
    panic!("Problem parsing line \"{}\"", line);
}

fn build_stack(originals: &HashMap<i32, &Card>, coll: &Vec<Card>) -> Vec<Card> {
    let mut stack: Vec<Card> = Vec::new();
    for p in coll {
        stack.push(Card {
            id: p.id,
            wins: p.wins,
            proc: true,
        });
        if p.wins > 0 && !p.proc {
            let index = p.id + 1;
            for i in index..index + p.wins {
                let loc = originals.get(&i).unwrap();
                stack.push(Card {
                    id: loc.id,
                    wins: loc.wins,
                    proc: false,
                });
            }

        }
    }
    return stack
}

fn main() {
    use std::time::Instant;
    let input = fs::read_to_string("input/day4.txt").unwrap();
    let cards: Vec<Card> = input.lines().map(parse_card).collect();
    let mut originals: HashMap<i32, &Card> = HashMap::new() ;
    for card in &cards {
        originals.insert(card.id, card);
    }
    let now = Instant::now();
    let mut stack: Vec<Card> = build_stack(&originals, &cards);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    let mut cont: bool = stack.iter().any(|c| !c.proc);
    while cont {
        let now = Instant::now();
        stack = build_stack(&originals, &stack);
        // print_stack(&stack, false);
        cont = stack.iter().any(|c| !c.proc);
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }
    println!("result = {}", stack.len());
}
