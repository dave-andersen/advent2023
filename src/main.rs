use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const BASEDIR: &str = "/Users/dga/rust/advent2023";
const INPUTDIR: &str = "inputs";

fn input_file(puzzle: &str) -> String {
    format!("{BASEDIR}/{INPUTDIR}/{puzzle}.txt")
}

fn open_buffered(filename: &str) -> BufReader<File> {
    BufReader::new(File::open(filename).unwrap())
}

fn a1(part_two: bool) {
    let input = input_file("a1.1");
    let mut patterns = vec![
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine",
    ];
    let values = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    if !part_two {
        patterns.truncate(10);
    }

    let ac = aho_corasick::AhoCorasick::new(patterns).unwrap();

    let tot: u32 = open_buffered(&input)
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let digits: Vec<_> = ac
                .find_overlapping_iter(&line)
                .map(|pid| values[pid.pattern().as_usize()])
                .collect();
            if digits.is_empty() {
                0
            } else {
                digits[0] * 10 + digits[digits.len() - 1]
            }
        })
        .sum();
    println!("Total: {tot}");
}

fn main() {
    a1(false);
    a1(true);
}
