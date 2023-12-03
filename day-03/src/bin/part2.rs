#![feature(iter_map_windows)]

use std::ops::Range;

// fn any_check(b: &u8) -> bool {
//     return b.is_ascii_digit();
// }
//
// fn get_numbers_from_line(line: &&[u8], range: Range<usize>) -> Vec<i32> {
//     let line_range = &line[range.clone()];
//     let mut numbers: Vec<i32> = Vec::new();
//     if line_range.iter().any(any_check) {
//         if !line_range[0].is_ascii_digit() || !line_range[2].is_ascii_digit() {
//             // only one number
//             if line_range[0].is_ascii_digit() {
//                 // extends to left
//                 let mut start = range.start;
//                 if start == 0 {
//                     numbers.push(String::from_utf8(line[start..if line_range[1].is_ascii_digit() { 2 } else { 1 }].to_vec()).unwrap().parse::<i32>().unwrap());
//                 } else {
//                     while {}
//                     if range.start != 0 && line[range.start - 1].is_ascii_digit() {}
//                 }
//             } else {
//                 // extends to right
//             }
//         } else if line_range.iter().all(any_check) {
//             // only one number - but could extend in both directions
//         } else {
//             // two numbers - extends in both directions
//         }
//     }
//     return numbers;
// }

fn parse_line(line: &[u8]) -> Vec<(i32, Range<usize>)> {
    let mut numbers: Vec<(i32, Range<usize>)> = Vec::new();
    let mut first: Option<usize> = None;

    for i in 0..line.len() {
        // byte is digit
        if line[i].is_ascii_digit() {
            // first is not set - set first
            if !first.is_some() {
                first = Some(match i {
                    0 => 0,
                    _ => i,
                });
            }

            // line is not ended and next byte is digit - continue
            if i != line.len() - 1 && line[i + 1].is_ascii_digit() {
                continue;
            }

            numbers.push((String::from_utf8((*&line[first.unwrap()..i + 1]).to_vec()).unwrap().parse::<i32>().unwrap(), first.unwrap()..i + 1));

            first = None;
        }
    }

    return numbers;
}

fn main() {
    let sum: i32 = include_bytes!("../input.txt").split(|b| b == &b'\n').map_windows(|[last, line, next]| {
        let mut sum = 0;
        let last_numbers = parse_line(last);
        let line_numbers = parse_line(line);
        let next_numbers = parse_line(next);

        for i in 0..line.len() {
            if line[i] != b'*' {
                continue;
            }

            let line_range = match i {
                0 => 0,
                i => i - 1,
            }..if i == line.len() - 1 { i + 1 } else { i + 2 };

            let last_adjacent_numbers = last_numbers.iter().filter_map(|(number, range)| {
                // check if ranges overlap
                return if line_range.contains(&range.start) || line_range.contains(&(range.end - 1)) { Some(number) } else { None };
            });
            let next_adjacent_numbers = next_numbers.iter().filter_map(|(number, range)| {
                // check if ranges overlap
                return if line_range.contains(&range.start) || line_range.contains(&(range.end - 1)) { Some(number) } else { None };
            });

            let number_to_right = if i == line.len() - 1 { None } else {
                line_numbers.iter()
                    .find_map(|(number, range)| if range.contains(&(&i + 1)) { Some(number) } else { None })
            };
            let number_to_left = if i == 0 { None } else {
                line_numbers.iter()
                    .find_map(|(number, range)| if range.contains(&(&i - 1)) { Some(number) } else { None })
            };

            let mut numbers = last_adjacent_numbers.chain(next_adjacent_numbers).chain(number_to_right).chain(number_to_left);

            if numbers.clone().count() != 2 { continue; }

            sum += numbers.nth(0).unwrap() * numbers.nth(0).unwrap();
        }
        return sum;
    }).sum();

    println!("{}", sum);
}
