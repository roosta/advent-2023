use std::fs;
use advent_2023::parse_line;
use std::collections::HashMap;
use std::ops::Range;

#[derive(Debug)]
#[allow(warnings)]
struct MapRange {
    src_start: i64,
    src_end: i64,
    dest_start: i64,
    dest_end: i64,
}

#[derive(Debug)]
struct Map {
    title: String,
    data: Vec<MapRange>
}

// Return a dest number, either from the ranges defined in maps, or fall back to input target
fn target_dest(maps: &Vec<MapRange>, target: i64) -> i64 {
    for mr in maps {
        if target >= mr.src_start && target < mr.src_end {
            return mr.dest_start - mr.src_start + target;
        } else {
            continue
        }
    }
    return target;
}

// Finds a location for a given seed
fn find_location(
    almanac: &HashMap<String, Vec<MapRange>>,
    seed: i64
) -> i64 {
    let keys = [
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location"
    ];
    let mut target = seed;
    for key in keys {
        let maps = almanac.get(key).unwrap();
        target = target_dest(&maps, target);

    }
    return target;
}

// Use a binary search to find the lowest location given a range of seeds
fn find_lowest_location(
    seed_range: &Range<i64>,
    alamanc: &HashMap<String, Vec<MapRange>>,
    init_loc: i64
) -> i64 {
    let mut left = seed_range.start;
    let mut right = seed_range.end - 1;
    let mut min_loc = init_loc;

    while left < right {
        let mid = left + (right - left) / 2;
        let loc = find_location(alamanc, mid);
        if loc < min_loc {
            min_loc = loc;
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    min_loc
}


fn parse_block(block: &Vec<&str>) -> Map {
    let mut iter = block.iter();
    let title = iter.next().unwrap().to_owned().to_owned();
    let title = title.replace(" map:", "");

    let parsed_lines: Vec<Vec<i64>> = iter.map(|line| parse_line(line)).collect();
    let mut data: Vec<MapRange> = Vec::new();
    for line in &parsed_lines {
        if let [dest, src, n] = line[..] {
            data.push(MapRange {
                src_start: src,
                src_end: src + n,
                dest_start: dest,
                dest_end: dest + n
            })
        }
    }
    Map { title, data }
}

// Transform the seed notation in input file to ranges
fn seeds_to_ranges(seeds: Vec<i64>) -> Vec<Range<i64>> {
    seeds.chunks(2).map(|pair| {
        let [start, n] = pair else { panic!("No pair found for {:#?}", seeds) };
        *start..start + n
    }).collect()
}

fn main() {

    // Parse input
    let input = fs::read_to_string("input/day5.txt").unwrap();
    let mut lines = input.lines().peekable();
    let seeds = parse_line(lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap());
    let seeds = seeds_to_ranges(seeds);
    lines.next(); // skip blank line

    let mut block = Vec::new();
    let mut almanac: HashMap<String, Vec<MapRange>> = HashMap::new();
    while let Some(line) = lines.next() {
        if lines.peek().is_none() {
            block.push(line);
            let b = parse_block(&block);
            almanac.insert(b.title, b.data);
        } else if line.is_empty() {
            let b = parse_block(&block);
            almanac.insert(b.title, b.data);
            block = Vec::new();
        } else {
            block.push(line);
        }
    }

    // Calculate result
    let mut min_loc = std::i64::MAX;
    for seed_range in seeds {
        let val = find_lowest_location(&seed_range, &almanac, min_loc);
        if val < min_loc {
            min_loc = val;
        }
    }
    println!("lowest location: {:#?}", min_loc);
}
