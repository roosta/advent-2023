use std::fs;
use std::collections::HashMap;

const RED: i32 = 12;
const BLUE: i32 = 14;
const GREEN: i32 = 13;

fn get_number(line: &str) -> i32 {
    let digits = line.matches(char::is_numeric);
    let s = digits.fold(String::new(), | acc, s |  acc + s );
    let n: i32 = s.parse().unwrap();
    return n
}

fn get_color(line: &str) -> String {
    let colors = ["red", "green", "blue"];
    for color in colors {
        if line.contains(color) {
            return color.to_owned()
        }
    }
    panic!("Couldn't find a color!")
}

fn parse_game(line: &str) -> Option<i32> {
    let v: Vec<&str> = line.split(':').collect();
    if let [id, rest] = &v[..] {
        let id = get_number(id);
        let reveal = rest.split(';');
        for set in reveal {
            let mut result = HashMap::from([
                ("red",   RED),
                ("green", GREEN),
                ("blue",  BLUE),
            ]);
            for color_str in set.split(',') {
                let n = get_number(color_str);
                let c = get_color(color_str);
                let target = result.get_mut(&c as &str).unwrap();
                *target -= n;
                if *target < 0 {
                    return None
                }
            }

        }
        return Some(id)
    } else {
        panic!("Failed to parse line \"{}\"", line);
    }
}

pub fn main() {
    let input = fs::read_to_string("input/day2.txt").unwrap();
    let games: Vec<Option<i32>> = input.lines().map(parse_game).collect();
    let mut result = 0;
    for game in games {
        if let Some(id) = game {
            result += id

        }
    }
    println!("result = {:#?}", result);
}
