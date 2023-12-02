use std::fs;

#[derive(Debug)]
struct Number {
    i: i32,
    alpha: char,
    word: &'static str,
}

const NUM: [Number; 9] = [
    Number { i: 1, alpha: '1', word: "one" },
    Number { i: 2, alpha: '2', word: "two" },
    Number { i: 3, alpha: '3', word: "three" },
    Number { i: 4, alpha: '4', word: "four" },
    Number { i: 5, alpha: '5', word: "five" },
    Number { i: 6, alpha: '6', word: "six" },
    Number { i: 7, alpha: '7', word: "seven" },
    Number { i: 8, alpha: '8', word: "eight" },
    Number { i: 9, alpha: '9', word: "nine" },
];

fn find_digit(line: &str) -> Option<Number> {
    let mut pos = line.len();
    let mut out: Option<Number> = None;
    for n in NUM {
        let word = line.find(n.word);
        let alpha = line.find(n.alpha);
        match (alpha, word) {
            (Some(a), Some(w)) => {
                if a < w && pos < a {
                    pos = a;
                    out = Some(n);
                } else if w < a && pos < w {
                    pos = w;
                    out = Some(n);
                }
            },
            (Some(a), None) => {
                if a < pos {
                    pos = a;
                    out = Some(n)

                }
            },
            (None, Some(w)) => {
                if w < pos {
                    pos = w;
                    out = Some(n);
                }
            },
            (None, None) => continue
        };
    }
    out
}

fn rfind_digit(line: &str) -> Option<Number> {
    let mut pos = 0;
    let mut out: Option<Number> = None;
    for n in NUM {
        let word = line.rfind(n.word);
        let alpha = line.rfind(n.alpha);
        match (alpha, word) {
            (Some(a), Some(w)) => {
                if a > w && pos > a {
                    pos = a;
                    out = Some(n);
                } else if w > a && pos > w {
                    pos = w;
                    out = Some(n);
                }
            },
            (Some(a), None) => {
                if a > pos {
                    pos = a;
                    out = Some(n)

                }
            },
            (None, Some(w)) => {
                if w > pos {
                    pos = w;
                    out = Some(n);
                }
            },
            (None, None) => continue
        };
    }
    out
}
pub fn main() {
    let input = fs::read_to_string("input/day1/part2.txt").unwrap();
    let result: Option<i32> = input.lines().map(|line| {
        let left = match find_digit(line) {
            Some(d) => d,
            None => panic!("Can't find left digit in line: {}", line)
        };
        let right = match rfind_digit(line) {
            Some(d) => d,
            None => panic!("Can't find right digit in line: {}", line)

        };
        return left.i * 10 + right.i
    }).reduce(|acc, n| acc + n);
    println!("result = {:#?}", result.unwrap());
}
