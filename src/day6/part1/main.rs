use std::fs;
use advent_2023::parse_line;
use std::iter::zip;
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

fn run_opts(max_time: i64, record_distance: i64) -> i64  {
    let mut nwins = 0;
    for speed in 0..=max_time {
        let time = max_time - speed;
        let factor = speed % max_time;
        let distance = time * factor;
        if distance > record_distance {
            nwins += 1;
        }
    }
    nwins
}

fn main() {
    let input = fs::read_to_string("input/day6.txt").unwrap();
    let game = parse_game(input);
    let iter = zip(game.time, game.distance);
    let mut result = 1;
    for (time, distance) in iter {
        result *= run_opts(time, distance);
    }
    println!("Result = {}", result);
}
