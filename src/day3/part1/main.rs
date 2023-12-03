use std::fs;
use std::collections::HashMap;

fn read_into(lines: Vec<&str>) -> Vec<Vec<char>> {
    let width = lines[0].len();
    let height = lines.len();
    let mut grid = vec![vec!['.'; width]; height];
    for (row, line) in lines.iter().enumerate() {
        for (col, symbol) in line.chars().enumerate() {
            grid[row][col] = symbol;
        }
    }
    grid
}

struct Part {
    number: i32,
    symbol: char,
}

fn neighbors() {

}

fn find_number(grid: &Vec<Vec<char>>, i: i32, j: i32) {
    for row in [-1, 0, 1] {
        for col in [-1, 0, 1] {
            if row != 0 && col != 0 {
                println!("{}, {}", i + row, j + col)
            }
        }
    }

}

// nw n ne
// w     e
// sw s se
fn main() {
    let input = fs::read_to_string("input/day3.txt").unwrap();
    // let parts: HashMap<char, Part> = HashMap::new();
    // for (i, line) in input.lines().enumerate() {
    //     for (j, c) in line.chars().enumerate() {
    //         if c != '.' && !char::is_digit(c, 10) {
    //
    //         }
    //
    //     }
    //     let numbers: Vec<&str> = line.split(|c| !char::is_digit(c, 10)).collect();
    //     let symbols: Vec<&str> = line.split(|c| char::is_digit(c, 10) || c == '.').collect();
    // }
    let grid = read_into(input.lines().collect());
    find_number(&grid, 1, 1);
    // for (i, row) in grid.iter().enumerate() {
    //     for (j, col) in row.iter().enumerate() {
    //         if !char::is_digit(*col, 10) && *col != '.' {
    //             find_number(&grid, i, j)
    //         }
    //     }
    // }
    // println!("{:#?}", grid);
}
