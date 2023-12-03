use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct Word {
    value: char,
    index: usize
}

fn find_word(line: &str, dir: &str) -> Option<Word> {
    let mut result: Option<Word> = None;
    let numalpha = HashMap::from([
        ("one",   '1'),
        ("two",   '2'),
        ("three", '3'),
        ("four",  '4'),
        ("five",  '5'),
        ("six",   '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine",  '9')

    ]);
    for (key, value) in numalpha {
        let opt = if dir == "reverse" {
            line.rfind(key)
        } else {
            line.find(key)
        };
        match opt {
            Some(i) => {
                if let Some(r) = &result {
                    if dir == "forward" && i < r.index {
                        result = Some(Word { value, index: i })
                    } else if dir == "reverse" && i > r.index {
                        result = Some(Word { value, index: i })
                    }
                } else {
                    result = Some(Word { value, index: i })
                }

            }
            _ => ()


        }
    }
    result
}

fn find_digits(line: &str) -> String {
    let mut result = String::new();

    for dir in ["forward", "reverse"] {
        let digit = if dir == "forward" {
            line.find(|c: char| c.is_digit(10))
        } else {
            line.rfind(|c: char| c.is_digit(10))
        };

        let word = find_word(line, dir);
        match (digit, word) {
            (Some(d), Some(w)) => {
                let test = if dir == "forward" { d < w.index } else { d > w.index };
                if test {
                    result.push(line.chars().nth(d).unwrap())
                } else {
                    result.push(w.value)
                }
            },
            (Some(d), None) => result.push(line.chars().nth(d).unwrap()),
            (None, Some(w)) => result.push(w.value),
            _ => panic!("Didn't find any number on line \"{}\"", line)
        }
    }

    result
}
pub fn main() {
    let input = fs::read_to_string("input/day1.txt").unwrap();
    let mut result = 0;
    for line in input.lines() {
        let n = find_digits(line);
        let p: i32 = n.parse().unwrap();
        result += p;
    }
    println!("result = {:#?}", result);
}
