use std::fs;
use advent_2023::parse_line;
// use std::collections::HashMap;

#[derive(Debug)]
struct Game {
    time: Vec<i64>,
    distance: Vec<i64>
}

fn parse_game(input: String) -> Game {
    let mut iter = input.lines();
    let time_str = iter.next().unwrap().split(':').last().unwrap();
    let distance_str = iter.next().unwrap().split(':').last().unwrap();
    return Game {
        time: parse_line(time_str),
        distance: parse_line(distance_str)
    }

}

fn main() {
    let input = fs::read_to_string("input/day6.txt").unwrap();
    let game = parse_game(input);
}
