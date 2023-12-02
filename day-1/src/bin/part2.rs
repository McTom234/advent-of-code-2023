const NUMS: &[&[u8]] = &[
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

pub fn main() {
    let mut i = 0;
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|b| b == &b'\n')
            .map(|line| {
                i += 1;
                let first = (0..line.len()).find_map(|i| num(line, i));
                let second = (0..line.len()).rev().find_map(|i| num(line, i));
                println!("{}: {} {}", i, first.is_some(), second.is_some());
                first.unwrap() * 10 + second.unwrap()
            })
            .sum::<usize>()
    );
}

#[inline(always)]
fn num(line: &[u8], i: usize) -> Option<usize> {
    line[i]
        .is_ascii_digit()
        .then_some((line[i] - b'0') as usize)
        .or(NUMS
            .iter()
            .enumerate()
            .find(|(_, name)| line[i..].starts_with(name))
            .map(|(num, _)| num + 1))
}