use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};

fn main() {
    match read_lines() {
        Ok(lines) => {
            let mut sum = 0;
            for line in lines {
                let line = match line {
                    Ok(line) => line,
                    Err(_) => panic!("Cannot read line.")
                };

                let mut digits = String::new();
                let chars = line.chars();
                // because rev() cannot be used after iterator moved
                // because rev() moves iterator, we have to clone the iterator
                let reverse_chars = chars.clone().rev();

                // parse line from the front and parse first number
                for char in chars {
                    if char.is_digit(10) {
                        digits.push(char);
                        break;
                    }
                }

                // parse line from the back and parse first number
                for char in reverse_chars {
                    if char.is_digit(10) {
                        digits.push(char);
                        break;
                    }
                }

                // add both numbers together
                let num = digits.parse::<i32>().unwrap();

                // add to sum
                sum += num;
            }
            println!("{}", sum);
        }
        Err(_) => panic!("Cannot read file.")
    }
}

fn read_lines() -> Result<Lines<BufReader<File>>> {
    let file = File::open("../input.txt");
    return match file {
        Ok(file) => Ok(BufReader::new(file).lines()),
        Err(e) => Err(e),
    };
}