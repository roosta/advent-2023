use std::fs;

pub fn main() {
    let input = fs::read_to_string("input/day1/part2.txt").unwrap();
    let result: Vec<i32> = input.lines().map(|line| {
        // let left = line.find(|c: char| c.is_digit(10)).unwrap();
        // let right = line.rfind(|c: char| c.is_digit(10)).unwrap();
        // let mut p = String::new();
        // p.push(line.chars().nth(left).unwrap());
        // p.push(line.chars().nth(right).unwrap());
        // let n: i32 = p.parse().unwrap();
        12
    }).collect();
    println!("result = {:#?}", result);
}
