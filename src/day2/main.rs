use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Game {
    id: u32,
    throws: Vec<Throw>,
}

#[derive(Debug)]
struct Throw {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let file = File::open("src/day2/input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let game_regex = Regex::new(r"Game\s(\d+):\s(.*)$").unwrap();
    let mut games: Vec<Game> = Vec::new();
    let mut sum: u32 = 0;

    for line in lines {
        if let Ok(cont) = line {
            let Some((_, [id, throws])) = game_regex.captures(&cont).map(|cap| cap.extract()) else { return; };
            let mut rolls: Vec<&str> = Vec::new();
            let mut game: Game = Game {
                id: id.parse().unwrap(),
                throws: Vec::new(),
            };
            let rolls_text_regex = Regex::new(r"([^;]+)").unwrap();
            for (_, [text]) in rolls_text_regex.captures_iter(throws).map(|c| c.extract()) {
                rolls.push(text);
            }
            for text in rolls {
                // Theres probably a better way, but who cares
                let red_regex = Regex::new(r"(\d+)\s*red").unwrap();
                let green_regex = Regex::new(r"(\d+)\s*green").unwrap();
                let blue_regex = Regex::new(r"(\d+)\s*blue").unwrap();
                let (_, [red]) = red_regex
                    .captures(text)
                    .map_or(("", ["0"]), |cap| cap.extract());
                let (_, [green]) = green_regex
                    .captures(text)
                    .map_or(("", ["0"]), |cap| cap.extract());
                let (_, [blue]) = blue_regex
                    .captures(text)
                    .map_or(("", ["0"]), |cap| cap.extract());
                game.throws.push(Throw {
                    red: red.parse::<u32>().unwrap_or(0),
                    green: green.parse::<u32>().unwrap_or(0),
                    blue: blue.parse::<u32>().unwrap_or(0),
                })
            }
            games.push(game);
        }
    }

    for game in games {
        println!("{:?}", game);
        sum = sum + game.id;
        for throw in game.throws {
            if throw.red > 12 || throw.green > 13 || throw.blue > 14 {
                sum = sum - game.id;
                break;
            }
        }
    }

    println!("Result: {}", sum);
}
