use std::cmp::{min, Ordering};
use std::collections::HashMap;

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let is_digit = |c: &char| -> bool { c.is_ascii_digit() };
            let first = line.chars().find(is_digit).unwrap();
            let last = line.chars().rev().find(is_digit).unwrap();
            ((first as u8 - b'0') as i32 * 10) + ((last as u8 - b'0') as i32)
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let digits = HashMap::from([
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9")
    ]);

    input
        .lines()
        .map(|line: &str| {
            let mut result = String::new();
            let mut it = line.char_indices();

            while let Some((i, c)) = it.next() {
                if c.is_ascii_digit() { result += &c.to_string() }
                let possible_digit = digits.keys().find(|d| {
                    let max_len = min(d.len(), line.len() - i);
                    (**d).cmp(&line[i..(i + max_len)]) == Ordering::Equal
                });

                if let Some(digit) = possible_digit {
                    result += digits[digit];
                    // There are cases like sevenine that are supposed to parse to 79
                    // so we don't want to advance past the last character in the spelt out digit
                    for _ in 2..digit.len() {
                        it.next();
                    }
                }
            }

            let first = result.chars().nth(0).unwrap();
            let last = result.chars().last().unwrap();
            ((first as u8 - b'0') as i32 * 10) + ((last as u8 - b'0') as i32)
        })
        .sum()
}
