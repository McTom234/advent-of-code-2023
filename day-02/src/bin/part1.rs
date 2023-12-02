use std::collections::HashMap;
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
            let mut game_map = HashMap::new();
            for line in lines {
                let line: String = match line {
                    Ok(line) => line,
                    Err(e) => panic!("Cannot read line. {}", e)
                };

                let game_split: Vec<&str> = line.split(": ").collect();

                let game_id: i32 = game_split[0].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();

                let game_results = game_split[1].split("; ");

                let mut rounds: Vec<Round> = Vec::new();

                for result in game_results {
                    let mut round = Round {
                        red: 0,
                        blue: 0,
                        green: 0,
                    };

                    for r in result.split(", ") {
                        let round_split = r.split(" ").collect::<Vec<&str>>();
                        let round_amount = round_split[0].parse::<i32>().unwrap();
                        let round_color = round_split[1];

                        match round_color {
                            "red" => round.red = round_amount,
                            "blue" => round.blue = round_amount,
                            "green" => round.green = round_amount,
                            _ => panic!("Unknown color")
                        }
                    }

                    rounds.push(round);
                }

                game_map.insert(game_id, rounds);
            }

            let mut total_ids = 0;
            'root: for (id, rounds) in game_map {
                for round in rounds {
                    if round.red > 12 || round.green > 13 || round.blue > 14 {
                        continue 'root;
                    }
                }
                total_ids += id;
            }
            println!("Total ids: {}", total_ids);
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
