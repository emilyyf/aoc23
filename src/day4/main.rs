use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

fn count_hits(x: Vec<u32>, y: Vec<u32>) -> u32 {
    let mut hits: u32 = 0;

    for i in x {
        if y.contains(&i) {
            hits += 1;
        }
    }

    hits
}

fn main() {
    let file = File::open("src/day4/input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut sum: u32 = 0;

    for line in lines {
        if let Ok(cont) = line {
            let line_regex = Regex::new(r"Card\s+(\d+):([^\|]+)\|(.*)").unwrap();
            let numbers_regex = Regex::new(r"(\d+)").unwrap();
            let mut winning_numbers: Vec<u32> = Vec::new();
            let mut my_numbers: Vec<u32> = Vec::new();
            let winning_numbers_text = line_regex.captures(&cont).unwrap().get(2).unwrap().as_str();
            let my_numbers_text = line_regex.captures(&cont).unwrap().get(3).unwrap().as_str();
            for (_, [number]) in numbers_regex
                .captures_iter(winning_numbers_text)
                .map(|c| c.extract())
            {
                winning_numbers.push(number.parse().unwrap());
            }
            for (_, [number]) in numbers_regex
                .captures_iter(my_numbers_text)
                .map(|c| c.extract())
            {
                my_numbers.push(number.parse().unwrap());
            }
            let hits = count_hits(winning_numbers, my_numbers);
            sum += 2_u32.pow(std::cmp::max(hits, 1) - 1) * std::cmp::min(1, hits);
        }
    }

    println!("Result: {}", sum);
}
