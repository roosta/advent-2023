use std::fs;
use advent_2023::parse_line;

fn target_dest(block: &Vec<&str>, target: i64) -> i64 {
    let mut iter = block.iter();
    iter.next(); // Skip heading
    for line in iter {
        let num = parse_line(line);
        if let [dest, src, n] = num[..] {
            if target >= src && target < src + n {
                 return dest - src + target;
            } else {
                continue
            }
        }
    }
    return target;
}

fn run_block(block: &Vec<&str>, targets: Vec<i64>) -> Vec<i64> {
    use std::time::Instant;
    let now = Instant::now();
    let mut result = Vec::new();
    for target in targets {
        result.push(target_dest(&block, target));
    }
    let elapsed = now.elapsed();
    println!("Block processed in: {:.2?}", elapsed);
    result
}

fn main() {
    let input = fs::read_to_string("input/day5.txt").unwrap();
    let mut lines = input.lines().peekable();
    let seeds = parse_line(lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap());

    lines.next(); // skip blank line
    let mut block = Vec::new();
    let mut t = seeds;
    while let Some(line) = lines.next() {
        if lines.peek().is_none() {
            block.push(line);
            t = run_block(&block, t);
            block = Vec::new();
        } else if line.is_empty() {
            t = run_block(&block, t);
            block = Vec::new();
        } else {
            block.push(line);
        }
    }
    println!("result = {:#?}", t.iter().min().unwrap());
}
