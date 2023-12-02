use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};

#[derive(Debug)]
struct Round {
    red: i32,
    blue: i32,
    green: i32,
}

fn main() {
    match read_lines() {
        Ok(lines) => {
            let mut games = Vec::new();
            for line in lines {
                let line: String = match line {
                    Ok(line) => line,
                    Err(e) => panic!("Cannot read line. {}", e)
                };

                let game_split = line.as_str().split_once(": ").unwrap();

                let game_results = game_split.1.split("; ");

                let mut round = Round {
                    red: 0,
                    blue: 0,
                    green: 0,
                };

                for result in game_results {

                    for r in result.split(", ") {
                        let round_split = r.split_once(" ").unwrap();
                        let round_amount = round_split.0.parse::<i32>().unwrap();
                        let round_color = round_split.1;

                        match round_color {
                            "red" => if round.red < round_amount { round.red = round_amount },
                            "blue" => if round.blue < round_amount { round.blue = round_amount },
                            "green" => if round.green < round_amount { round.green = round_amount },
                            _ => panic!("Unknown color")
                        }
                    }
                }

                games.push(round);
            }

            let mut power_sum = 0;
            for round in games {
                power_sum += round.red * round.blue * round.green;
            }
            println!("Sum of powers: {}", power_sum);
        }
        Err(e) => {
            panic!("Cannot read file. {}", e);
        }
    }
}

fn read_lines() -> Result<Lines<BufReader<File>>> {
    let file = File::open("src/input.txt");
    return match file {
        Ok(file) => Ok(BufReader::new(file).lines()),
        Err(e) => Err(e),
    };
}
