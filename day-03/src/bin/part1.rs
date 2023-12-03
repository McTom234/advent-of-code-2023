#![feature(iter_map_windows)]

fn main() {
    let any_check = |b: &u8| b != &b'.';
    let a: i32 = include_bytes!("../input.txt").split(|b| b == &b'\n').map_windows(|[last, line, next]| {
        let mut first: Option<usize> = None;

        let mut sum = 0;

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

                // line is ended or next byte is not digit
                let range = match first.unwrap() {
                    0 => 0,
                    f => f - 1,
                }..if i == line.len() - 1 { i + 1 } else { i + 2 };

                let last_line_contains = last[range.clone()].iter().any(any_check);

                let next_line_contains = next[range.clone()].iter().any(any_check);

                let previous_byte = if first.unwrap() == 0 { b'.' } else { line[first.unwrap() - 1] };
                let previous_byte_contains = any_check(&previous_byte);

                let next_byte = if i == line.len() - 1 { b'.' } else { line[i + 1] };
                let next_byte_contains = any_check(&next_byte);

                let number = String::from_utf8((*&line[first.unwrap()..i + 1]).to_vec()).unwrap();

                let number_counts = last_line_contains || next_line_contains || previous_byte_contains || next_byte_contains;

                dbg!(number.clone(), last_line_contains, next_line_contains, previous_byte_contains, next_byte_contains);

                if number_counts {
                    sum += number.parse::<i32>().unwrap();
                }

                first = None;
            }
        }

        return sum;
    }).sum();
    dbg!(a);
}
