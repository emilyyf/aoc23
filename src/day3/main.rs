use std::fs::File;
use std::io::{self, BufRead};

fn is_number(c: char) -> bool {
    ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(&c)
}

fn is_symbol(c: char) -> bool {
    !['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'].contains(&c)
}

fn get_full_number(start: usize, line: Vec<char>) -> (u32, usize) {
    let mut buf: Vec<char> = Vec::new();
    let mut i: usize = start;
    while i < 140 {
        if is_number(line[i]) {
            buf.push(line[i]);
        } else {
            break;
        }
        i += 1;
    }

    (
        buf.into_iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
            .concat()
            .parse::<u32>()
            .unwrap(),
        i - 1,
    )
}

fn is_part(first: usize, last: usize, line: usize, grid: Vec<Vec<char>>) -> bool {
    let first_line = if line > 0 { line - 1 } else { 0 };
    let first_num = if first > 0 { first - 1 } else { 0 };
    let last_line = if line < 139 { line + 1 } else { 139 };
    let last_num = if last < 139 { last + 1 } else { 139 };

    for i in first_line..=last_line {
        for j in first_num..=last_num {
            if is_symbol(grid[i][j]) {
                return true;
            }
        }
    }

    false
}

fn main() {
    let file = File::open("src/day3/input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut sum: u32 = 0;
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut i: usize = 0;
    let mut j: usize;

    for line in lines {
        if let Ok(cont) = line {
            let mut buf: Vec<char> = Vec::new();
            for byte in cont.bytes() {
                buf.push(byte as char);
            }
            grid.push(buf);
        }
    }

    while i < 140 {
        j = 0;
        while j < 140 {
            if is_number(grid[i][j]) {
                let (num, last) = get_full_number(j, grid[i].clone());
                if is_part(j, last, i, grid.clone()) {
                    sum += num;
                }
                j = last + 1;
            } else {
                j += 1;
            }
        }
        i += 1;
    }

    println!("Result: {}", sum);
}
