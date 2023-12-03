use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("src/day1/input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let re = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let mut numbers: Vec<i32> = Vec::new();
    for line in lines {
        if let Ok(cont) = line {
            let mut i: usize = 0;
            let mut ret: Vec<i32> = Vec::new();
            while i <= cont.len() {
                println!("{} - {}", cont.len(), i);
                if let Some(x) = re.find_at(&cont, i) {
                    let mat = match x.as_str() {
                        "one" => 1,
                        "two" => 2,
                        "three" => 3,
                        "four" => 4,
                        "five" => 5,
                        "six" => 6,
                        "seven" => 7,
                        "eight" => 8,
                        "nine" => 9,
                        a => a.parse::<i32>().unwrap(),
                    };
                    ret.push(mat);
                    i = x.start() + 1;
                } else {
                    break;
                }
            }
            numbers.push(ret.first().unwrap() * 10 + ret.last().unwrap());
            println!("line: {} ret: {:?}", cont, ret);
        }
    }
    let sum = numbers.iter().sum::<i32>();
    println!("Result: {}", sum);
}
