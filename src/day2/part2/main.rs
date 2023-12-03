use std::fs;
use std::collections::HashMap;

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

fn parse_game(line: &str) -> i32 {
    let v: Vec<&str> = line.split(':').collect();
    let mut result = HashMap::from([
        ("red",   1),
        ("green", 1),
        ("blue",  1),
    ]);
    if let [_, rest] = &v[..] {
        let reveal = rest.split(';');
        for set in reveal {
            for color_str in set.split(',') {
                let n = get_number(color_str);
                let c = get_color(color_str);
                let target = result.get_mut(&c as &str).unwrap();
                if n > *target {
                    *target = n
                }
            }

        }
    } else {
        panic!("Failed to parse line \"{}\"", line);
    }

    return result.values().fold(1, |acc, v| acc * v);

}

pub fn main() {
    let input = fs::read_to_string("input/day2.txt").unwrap();
    let games = input.lines().map(parse_game);
    let result = games.reduce(|acc, n| acc + n).unwrap();
    println!("result = {}", result);
}
