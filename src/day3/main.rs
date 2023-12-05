use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn is_number(c: char) -> bool {
    ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(&c)
}

fn is_star(c: char) -> bool {
    c == '*'
}

fn is_gear(i: usize, j: usize, grid: Vec<Vec<char>>) -> bool {
    let first_line = if i > 0 { i - 1 } else { 0 };
    let first_num = if j > 0 { j - 1 } else { 0 };
    let last_line = if i < 139 { i + 1 } else { 139 };
    let last_num = if j < 139 { j + 1 } else { 139 };
    let mut nums = HashSet::new();

    for i in first_line..=last_line {
        for j in first_num..=last_num {
            if is_number(grid[i][j]) {
                let (num, _) = get_full_number(j, grid[i].clone());
                nums.insert(num);
            }
        }
    }

    nums.len() == 2
}

fn get_gear_nums(i: usize, j: usize, grid: Vec<Vec<char>>) -> (u32, u32) {
    let first_line = if i > 0 { i - 1 } else { 0 };
    let first_num = if j > 0 { j - 1 } else { 0 };
    let last_line = if i < 139 { i + 1 } else { 139 };
    let last_num = if j < 139 { j + 1 } else { 139 };
    let mut points: Vec<u32> = Vec::new();
    let mut nums = HashSet::new();

    for i in first_line..=last_line {
        for j in first_num..=last_num {
            if is_number(grid[i][j]) {
                let (num, _) = get_full_number(j, grid[i].clone());
                nums.insert(num);
            }
        }
    }

    for i in nums {
        points.push(i);
    }

    (*points.get(0).unwrap(), *points.get(1).unwrap())
}

fn get_full_number(start: usize, line: Vec<char>) -> (u32, usize) {
    let mut buf: Vec<char> = Vec::new();
    let mut i: usize = start;

    while is_number(line[i - 1]) && i > 0 {
        i -= 1;
    }

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
            if is_star(grid[i][j]) {
                if is_gear(i, j, grid.clone()) {
                    let (num1, num2) = get_gear_nums(i, j, grid.clone());
                    sum += num1 * num2;
                }
            }
            j += 1;
        }
        i += 1;
    }

    println!("Result: {}", sum);
}
