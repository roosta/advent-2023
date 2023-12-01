use std::fs;

pub fn main() {
    let input = fs::read_to_string("input/day1/part1.txt").unwrap();
    let result: i32 = input.lines().map(|line| {
        let left = line.find(|c: char| c.is_digit(10)).expect("no left number found!");
        let right = line.rfind(|c: char| c.is_digit(10)).expect("no right number found!");
        let mut p = String::new();
        p.push(line.chars().nth(left).unwrap());
        p.push(line.chars().nth(right).unwrap());
        let n: i32 = p.parse().unwrap();
        n
    }).reduce(|a, b| a + b).unwrap();

    println!("result = {}", result);
}
