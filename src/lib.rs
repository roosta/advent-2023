// Parse number to i32
pub fn parse_n(s: &str) -> u32 {
    return s.parse().unwrap();
}

// Takes a string of space separated integers, returns a vector of parsed i32
pub fn parse_line(s: &str) -> Vec<u32> {
    return s
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(parse_n)
        .collect();
}

